---
name: tui
description: Use this skill when writing TUI (Text User Interface) code in Rust, handling user input, or when the user asks about TUI best practices, libraries, or patterns. Apply when on any *.rs files related to TUI development.
version: 1.0.0
---

## 🤖 Ratatui Development

This skill provides comprehensive guidelines for writing idiomatic, safe, and maintainable Tui applications in Rust using the `ratatui` crate.

### 1. Use `ratatui::init()` for Boilerplate
In 2026, we've moved away from manual terminal setup. 
* **Best Practice:** Use `let mut terminal = ratatui::init();` and `ratatui::restore();`.
* **Why:** This automatically handles the "Alternate Screen," "Raw Mode," and "Panic Hook" setup for you. If the agent writes 50 lines of `crossterm` boilerplate, it's using an outdated pattern.

### 2. Prefer `try_draw` for Error Handling
You are using `sqlx` and `ollama-rs`, which means your app is "fallible."
* **Best Practice:** Use `terminal.try_draw(|f| { ... })?` instead of the standard `.draw()`.
* **Why:** This allows you to use the `?` operator inside your UI code to bubble up errors if a widget fails to render or a calculation goes wrong.

### 3. The "Action" Pattern (Message Passing)
Since you're using **Tokio**, the AI agent should **never** share the `App` state across threads with a `Mutex<App>`.
* **Best Practice:** Use an **MPSC (Multi-Producer, Single-Consumer)** channel.
    * **Producer:** Input handlers and Tokio background tasks (Ollama/SQLx).
    * **Consumer:** The main TUI loop.
* **Why:** This keeps the UI loop non-blocking. If the AI is "thinking," the UI remains responsive (animations like your `throbber-widgets-tui` stay smooth).

### 4. Layout Constraints in 2026
* **Best Practice:** Use the `f.area()` method rather than the older `f.size()`.
* **Constraint Wisdom:** Always include a `Constraint::Min(0)` at the end of a layout list. This prevents the "sub-pixel" or rounding errors that used to cause TUIs to crash or flicker when the window was resized too small.

### 5. Performance: "Skip Render"
Since you're building an AI app, you'll likely have long streams of text.
* **Best Practice:** Implement a "dirty flag" or only call `terminal.draw` when an `Action` is received.
* **Why:** Calling `.draw()` 60 times a second when the AI is idle wastes CPU and battery. 

---

### Agent Checklist
1.  **Terminal Safety:** "Are we using `ratatui::init()` to ensure the terminal is restored if the code panics?"
2.  **Async Safety:** "Are we sending Ollama/SQLx results back via a `chan` instead of blocking the `draw` call?"
3.  **Modern Syntax:** "Are we using the `Widget` trait and `f.area()` instead of deprecated 0.2x patterns?"
