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

1.  **Read the Agent & Contributor Rules**: If you are using an AI coding agent (or running an automated workflow), read [AGENTS.md](AGENTS.md) first.
2.  **Match the Right Layer**: Confirm whether the issue belongs in Rust app code (`src/`), Cargo dependencies (`Cargo.toml` + `Cargo.lock`), Flatpak (`io.github._1dragon_xyz.tomodorange.yaml` / `.metainfo.xml`), AUR (`PKGBUILD`), or Nix (`flake.nix`).
3.  **Fork & Branch**: Create a focused branch for your fix or feature.
4.  **Implement**: Write clean, minimal Rust code and avoid unrelated dependency changes. If you modify `Cargo.toml`, you must also update `Cargo.lock`.
5.  **Verify Locally** (mandatory before opening a PR):
    ```bash
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo build --release --locked
    ```
    (Or run `just check && cargo build --release --locked`.)
6.  **Submit**: Fill out the [Pull Request Template](.github/PULL_REQUEST_TEMPLATE.md) completely, including exact checks run and whether the PR is AI-assisted. Unverified PRs or PRs with out-of-sync `Cargo.lock` files will be closed.

## Development Environment

We use `just` as our task runner.
*   `just run`: Build and start the app.
*   `just check`: Lint and format check.
*   `just clean`: Clear build artifacts.

---
*Stay fresh, stay focused.* 🍊
