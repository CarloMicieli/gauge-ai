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
use gauge_ai::tui::widgets::{render_command_failure, render_header_status, render_help};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

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

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let header_lines = render_header_status(&runtime, &runtime.health, 0, 0, area.width);
            let header_height = (header_lines.len() as u16).saturating_add(2);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(header_height),
                    Constraint::Min(4),
                    Constraint::Length(3),
                ])
                .split(area);

            let header = Paragraph::new(header_lines.join("\n"))
                .block(Block::default().title("Gauge.ai").borders(Borders::ALL));
            frame.render_widget(header, chunks[0]);

            let output = Paragraph::new(output_lines.join("\n"))
                .block(Block::default().title("Output").borders(Borders::ALL))
                .wrap(Wrap { trim: false });
            frame.render_widget(output, chunks[1]);

            let prompt = Paragraph::new(format!("> {input}"))
                .block(Block::default().title("Command").borders(Borders::ALL));
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
        Command::Quit => "/quit",
    }
}

fn current_epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}
