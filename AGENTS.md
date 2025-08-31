# Project Background

This project is a backend service written in Rust, making extensive use of asynchronous programming with the Tokio runtime.

## Coding Standards

- Use snake_case for variable and function names
- Indent code with 4 spaces
- Use triple slash `///` for documentation comments
- Follow Rust 2024 edition guidelines

## Dependencies

- tokio
- serde
- anyhow
- rand = "0.9"

## Design Principles

- Modular design
- Separation of business logic and interfaces
- Unit test coverage for core modules

## Code Quality Tools

- Use `cargo fmt` to automatically format all Rust source code and ensure consistent style.
- Use `cargo clippy` to lint code for potential errors, performance issues, and idiomatic Rust improvements.
- Treat Clippy warnings as errors and fix them promptly using `cargo clippy -- -D warnings`.
- Include these tools in CI pipelines to enforce code style and quality checks.

## Code push
- Use `export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7890` before git push origin main