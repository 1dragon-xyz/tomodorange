# Contributing to TomodOrange

Thank you for your interest in contributing to TomodOrange! We want to make this the best Pomodoro experience for the COSMIC™ desktop.

## Our Standards

### 1. Zero-Warning Policy
All code must pass `cargo clippy` and `cargo fmt` without warnings or errors.
*   Run `just check` before opening a pull request.

### 2. Conventional Commits
We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for our commit messages:
*   `feat:` for new features.
*   `fix:` for bug fixes.
*   `docs:` for documentation changes.
*   `refactor:` for code changes that neither fix a bug nor add a feature.

### 3. Native First
Every UI addition should use `libcosmic` primitives. Avoid custom CSS or non-standard widgets unless absolutely necessary for the "TomodOrange" aesthetic.

## How to Contribute

1.  **Find an Issue**: Look for issues labeled `good-first-issue`.
2.  **Fork & Branch**: Create a branch for your fix/feature.
3.  **Implement**: Write clean, commented Rust code.
4.  **Test**: Ensure the app builds and runs as expected.
5.  **Submit**: Open a Pull Request with a clear description of the changes.

## Development Environment

We use `just` as our task runner.
*   `just run`: Build and start the app.
*   `just check`: Lint and format check.
*   `just clean`: Clear build artifacts.

---
*Stay fresh, stay focused.* 🍊
