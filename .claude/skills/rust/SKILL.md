---
name: rust
description: Use this skill when writing Rust code, implementing domain logic, handling errors in Rust, or when the user asks about Rust best practices, Cargo commands, or Rust-specific patterns. Apply when on any *.rs files.
version: 1.0.0
---

# Rust Development Standards

This skill provides comprehensive guidelines for writing idiomatic, safe, and maintainable Rust code.

## When This Skill Applies

Use this skill when:

- Writing or modifying Rust code
- Implementing backend business logic, domain models, or infrastructure
- Handling errors, async operations, or database interactions
- Working with Tauri commands and state management
- User mentions: Rust, Cargo, backend, domain logic, error handling

## Project Context

- **Workspace Root**: `src` contains `Cargo.toml`
- **Execution**: All `cargo` commands must run from `src/`
- **Architecture**: Clean Architecture with domain-driven design

## Mandatory Workflow (The Execution Loop)

Before marking any Rust task as complete, you MUST execute this sequence:

1. **PLAN**: Detail the logic in chat. Mention specific crates (e.g., `sqlx`, `thiserror`)
2. **EXECUTE**: Implement logic in `src/`
3. **DOCUMENT**: Add `///` docstrings to all public APIs
4. **FORMAT**: Run `cargo fmt`
5. **VERIFY**: You MUST run and pass:
   - `cargo check`
   - `cargo clippy` (treat warnings as errors)
   - `cargo test`
   - If any command fails, read the compiler error, fix the code, and re-run verification
   - **Do not skip verification**

## Core Principles

### Safety & Type System

- Prioritize readability, safety, and maintainability
- Use strong typing and leverage Rust's ownership system
- Write code that compiles without warnings
- Handle errors gracefully using `Result<T, E>` with meaningful error messages
- Use `Option<T>` for values that may or may not exist

### Code Organization

- Organize by bounded contexts using modules
- Use `mod` and `pub` to encapsulate logic
- Break down complex functions into smaller, manageable functions
- Follow RFC 430 naming conventions
- Use clear and descriptive names for all items

### Documentation

- Document all public APIs with rustdoc (`///` comments)
- Include examples in documentation
- Document error conditions, panic scenarios, and safety considerations
- Explain algorithm approaches and design decisions
- Use `#[doc(hidden)]` for implementation details

## Patterns to Follow

### Error Handling

- Use `Result<T, E>` for recoverable errors, `panic!` only for unrecoverable ones
- Prefer `?` operator over `unwrap()` or `expect()`
- Create custom error types using `thiserror`
- Provide meaningful error messages with context
- Validate function arguments and return appropriate errors

### Ownership & Borrowing

- Prefer borrowing (`&T`) over cloning unless ownership transfer is necessary
- Use `&mut T` when modifying borrowed data
- Explicitly annotate lifetimes when compiler cannot infer
- Use `Rc<T>` for single-threaded, `Arc<T>` for thread-safe reference counting
- Use `RefCell<T>` for single-threaded, `Mutex<T>`/`RwLock<T>` for multi-threaded interior mutability

### Async & Concurrency

- Structure async code using `async/await` and `tokio`
- Handle async errors properly
- Use appropriate synchronization primitives

### Type Safety

- Implement common traits where appropriate: `Copy`, `Clone`, `Debug`, `PartialEq`, `Default`
- Use newtypes for static distinctions
- Prefer specific types over generic `bool` parameters
- Use enums over flags for type safety
- Implement standard conversion traits: `From`, `AsRef`, `AsMut`

### Performance

- Use iterators instead of index-based loops (faster and safer)
- Prefer `&str` over `String` for function parameters when ownership isn't needed
- Prefer borrowing and zero-copy operations to avoid unnecessary allocations
- Avoid premature `collect()`, keep iterators lazy
- Use builders for complex object creation

## Patterns to Avoid

- **Don't** use `unwrap()` or `expect()` unless absolutely necessary
- **Don't** panic in code—return `Result` instead
- **Don't** rely on global mutable state—use dependency injection
- **Don't** create deeply nested logic—refactor with functions or combinators
- **Don't** use `unsafe` unless required and fully documented
- **Don't** overuse `clone()`—prefer borrowing
- **Don't** make unnecessary allocations

## API Design Guidelines

### Common Traits

Eagerly implement where appropriate:

- `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`
- `Hash`, `Debug`, `Display`, `Default`
- `From`, `AsRef`, `AsMut`
- Collections: `FromIterator`, `Extend`

### Future Proofing

- Structs should have private fields
- Use sealed traits to protect against downstream implementations
- Functions should validate their arguments
- All public types must implement `Debug`

### Type Safety

- Functions with clear receiver should be methods
- Only smart pointers should implement `Deref`/`DerefMut`
- Arguments should convey meaning through types

## Testing

- Write comprehensive unit tests using `#[cfg(test)]` modules
- Use test modules alongside the code they test
- Write integration tests in `tests/` directory
- Include examples in rustdoc that use `?` operator, not `unwrap()`
- Test edge cases and error conditions

## Code Style

- Follow Rust Style Guide and use `rustfmt`
- Keep lines under 100 characters when possible
- Use `cargo clippy` to catch common mistakes
- Place documentation immediately before items using `///`

## Dependencies

Common crates in this project:

- **Error handling**: `thiserror`, `anyhow`
- **Serialization**: `serde`, `serde_json`
- **Database**: `sqlx` (with SQLite, tokio runtime)
- **Async**: `tokio`, `async-trait`

## Quality Checklist

Before submitting code, ensure:

### Core Requirements

- [ ] **Naming**: Follows RFC 430 naming conventions
- [ ] **Traits**: Implements `Debug`, `Clone`, `PartialEq` where appropriate
- [ ] **Error Handling**: Uses `Result<T, E>` with meaningful error types
- [ ] **Documentation**: All public items have rustdoc comments with examples
- [ ] **Testing**: Comprehensive test coverage including edge cases

### Safety and Quality

- [ ] **Safety**: No unnecessary `unsafe`, proper error handling
- [ ] **Performance**: Efficient use of iterators, minimal allocations
- [ ] **API Design**: Functions are predictable, flexible, and type-safe
- [ ] **Future Proofing**: Private fields in structs, sealed traits where appropriate
- [ ] **Tooling**: Passes `cargo fmt`, `cargo clippy`, and `cargo test`

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [RFC 430: Naming Conventions](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md)
- [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
