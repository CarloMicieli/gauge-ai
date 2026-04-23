## 🤖 AI Development Instructions

### 🧠 Operational Strategy
* **Plan Before Action:** Before writing or modifying code, provide a brief architectural plan. Outline which components are affected, how the state will change, and any potential impacts on the `Action` enum or event loop.
* **Commit Style:** Use **Conventional Commits** for all changes. Use simplified types like `feat:`, `fix:`, `docs:`, or `refactor:`. Do not include scopes (e.g., use `feat: add logic` instead of `feat(ui): add logic`).

### Core Architecture
* **Pattern:** Use a **Component-based architecture** with a central `Action` enum for state changes.
* **Concurrency:** All I/O (Ollama API, SQLx, File System) **must** be executed in `tokio::spawn` tasks. Results must be sent back to the main loop via `tokio::sync::mpsc`.
* **Terminal Lifecycle:** Use `ratatui::init()` and `ratatui::restore()` to manage raw mode and the alternate screen. Always implement a panic hook.

### Rendering & Layout
* **Immediate Mode:** Keep the `draw` function pure. No state mutation or I/O inside `terminal.draw`.
* **Area Management:** Use `f.area()` (Ratatui v0.30+) for layout calculations.
* **Responsiveness:** Use `Constraint::Percentage` or `Constraint::Fill` to ensure the TUI scales with terminal resizing.

### Dependency & Environment Rules
* **System Libs:** This project relies on `libchafa-dev`, `libglib2.0-dev`, and `libcairo2-dev`. If adding crates that wrap C libraries, verify `pkg-config` compatibility.
* **Cargo:** Strictly adhere to versions defined in `Cargo.toml`. 

### Error Handling
* Avoid `unwrap()` or `expect()`.
* Use the `?` operator and `anyhow` or `thiserror` for meaningful error propagation.
* In the TUI loop, catch errors and transition the app to an `ErrorState` rather than crashing the process.

### Code Style
* Follow standard Rust idioms (Clppy is enforced in CI).
* Document public methods in components.
* Prefer `impl Widget` for UI components to keep the rendering logic modular.



Here is the finalized instruction set for your repository. I have added the requirement for planning and the simplified conventional commit style.

