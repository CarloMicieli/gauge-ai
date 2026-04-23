use std::io;
use std::process::ExitCode;
use std::time::Instant;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use gauge_ai::ai::health::HealthStatus;
use gauge_ai::ai::knowledge_base::OllamaHealthState;
use gauge_ai::app::commands::{Command, command_error_message, parse};
use gauge_ai::app::config::AppConfig;
use gauge_ai::app::error::AppResult;
use gauge_ai::app::logging::init_logging;
use gauge_ai::app::perf::log_startup_timing;
use gauge_ai::app::state::RuntimeState;
use gauge_ai::tui::logo::pixel_sprite_lines;
use gauge_ai::tui::widgets::{render_command_failure, render_help, render_split_header};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Alignment;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

fn main() -> ExitCode {
    init_logging();
    let startup_started_at = Instant::now();

    if is_graceful_quit_requested() {
        println!("Gauge.ai shutdown completed.");
        return ExitCode::SUCCESS;
    }

    match run_startup(startup_started_at) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[gauge-ai] startup failed: {err}");
            ExitCode::FAILURE
        }
    }
}

fn is_graceful_quit_requested() -> bool {
    matches!(std::env::args().nth(1).as_deref(), Some("/quit" | "/exit"))
}

fn run_startup(startup_started_at: Instant) -> gauge_ai::app::error::AppResult<()> {
    let config = AppConfig::load()?;

    std::fs::create_dir_all(&config.data_dir)?;
    std::fs::create_dir_all(&config.cache_dir)?;

    run_tui_loop()?;
    let _ = log_startup_timing(startup_started_at);

    Ok(())
}

fn run_tui_loop() -> AppResult<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_tui_event_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_tui_event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> AppResult<()> {
    let now = current_epoch_secs();
    let health = HealthStatus {
        state: OllamaHealthState::Checking,
        missing_models: Vec::new(),
        last_error: None,
        last_checked_epoch_secs: now,
    };
    let mut runtime = RuntimeState::new(health);
    let mut input = String::new();
    let mut output_lines = vec![
        "Gauge.ai TUI started.".to_string(),
        "Type /help for available commands.".to_string(),
    ];
    let background_style = Style::default().bg(Color::Rgb(0, 0, 0));
    let primary_style = Style::default()
        .fg(Color::Rgb(255, 121, 76))
        .bg(Color::Rgb(0, 0, 0));
    let subtext_style = Style::default().fg(Color::DarkGray).bg(Color::Rgb(0, 0, 0));
    let locomotive_red = Style::default()
        .fg(Color::Rgb(220, 40, 40))
        .bg(Color::Rgb(0, 0, 0));
    let sprite_white = Style::default()
        .fg(Color::Rgb(255, 245, 245))
        .bg(Color::Rgb(0, 0, 0));
    let checking_icon_style = Style::default()
        .fg(Color::Rgb(172, 132, 255))
        .bg(Color::Rgb(0, 0, 0));
    let grounded_icon_style = Style::default()
        .fg(Color::Rgb(255, 214, 10))
        .bg(Color::Rgb(0, 0, 0));

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(Block::default().style(background_style), area);
            let header_sections = render_split_header(&runtime, &runtime.health, 0, 0, area.width);
            let right_content_height = 3;
            let left_content_height = if area.width >= 72 {
                pixel_sprite_lines(locomotive_red, sprite_white).len()
            } else {
                header_sections.left_ascii.len()
            };
            let top_content_height = left_content_height.max(right_content_height) as u16;
            let header_height = top_content_height.saturating_add(2);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(header_height),
                    Constraint::Min(4),
                    Constraint::Length(3),
                ])
                .split(area);

            let header_block = Block::default()
                .title(" Gauge.ai ")
                .title_style(primary_style.add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(primary_style);
            let top_inner_area = header_block.inner(chunks[0]);
            frame.render_widget(header_block, chunks[0]);

            let top_columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(top_inner_area);

            if area.width >= 72 {
                let left_inner = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(14),
                        Constraint::Length(2),
                        Constraint::Min(0),
                    ])
                    .split(top_columns[0]);

                let logo =
                    Paragraph::new(Text::from(pixel_sprite_lines(locomotive_red, sprite_white)))
                        .alignment(Alignment::Left);
                frame.render_widget(logo, left_inner[0]);

                let gaugeai = Paragraph::new(header_sections.right_banner.join("\n"))
                    .style(primary_style.add_modifier(Modifier::BOLD))
                    .alignment(Alignment::Left);
                frame.render_widget(gaugeai, left_inner[2]);
            } else {
                let left_lines: Vec<Line<'static>> = header_sections
                    .left_ascii
                    .iter()
                    .map(|line| Line::styled(line.clone(), locomotive_red))
                    .collect();
                let left_ascii = Paragraph::new(Text::from(left_lines)).alignment(Alignment::Left);
                frame.render_widget(left_ascii, top_columns[0]);
            }

            let right_block = Block::default()
                .borders(Borders::LEFT)
                .border_style(primary_style);
            let right_inner_area = right_block.inner(top_columns[1]);
            frame.render_widget(right_block, top_columns[1]);

            let right_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Min(0),
                ])
                .split(right_inner_area);

            let status_lines = vec![
                Line::from(vec![
                    Span::styled("✦ ", checking_icon_style),
                    Span::styled(header_sections.checking_line, subtext_style),
                ]),
                Line::from(vec![
                    Span::styled("▦ ", grounded_icon_style),
                    Span::styled(header_sections.grounded_line, subtext_style),
                ]),
            ];
            let status = Paragraph::new(Text::from(status_lines));
            frame.render_widget(status, right_layout[1]);

            let output = Paragraph::new(output_lines.join("\n"))
                .style(subtext_style)
                .block(
                    Block::default()
                        .title(" Console ")
                        .title_style(primary_style)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(primary_style),
                )
                .wrap(Wrap { trim: false });
            frame.render_widget(output, chunks[1]);

            let prompt = Paragraph::new(format!("> {input}"))
                .style(primary_style)
                .block(
                    Block::default()
                        .title(" Command ")
                        .title_style(primary_style)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(primary_style),
                );
            frame.render_widget(prompt, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(120))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        let submitted = input.trim().to_string();
                        input.clear();

                        if submitted.is_empty() {
                            output_lines.push("hint: enter a slash command like /help".to_string());
                        } else {
                            output_lines.push(format!("> {submitted}"));
                            match parse(&submitted) {
                                Ok(Command::Help) => output_lines.extend(render_help()),
                                Ok(Command::Clear) => output_lines.clear(),
                                Ok(Command::Quit) => break,
                                Ok(command) => {
                                    let command_name = command_label(&command);
                                    output_lines.extend(render_command_failure(
                                        command_name,
                                        "interactive execution wiring is pending",
                                    ));
                                }
                                Err(error) => output_lines.push(command_error_message(&error)),
                            }
                        }

                        trim_output_lines(&mut output_lines, 120);
                    }
                    _ => {}
                }
            }
        }

        runtime.tick_logo();
    }

    Ok(())
}

fn trim_output_lines(lines: &mut Vec<String>, max_lines: usize) {
    if lines.len() <= max_lines {
        return;
    }
    let excess = lines.len() - max_lines;
    lines.drain(0..excess);
}

fn command_label(command: &Command) -> &'static str {
    match command {
        Command::Help => "/help",
        Command::ListScraper => "/list-scraper",
        Command::Scrape { .. } => "/scrape",
        Command::Latest { .. } => "/latest",
        Command::Query { .. } => "/query",
        Command::Export { .. } => "/export",
        Command::Setup => "/setup",
        Command::Clear => "/clear",
        Command::Quit => "/quit",
    }
}

fn current_epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}
