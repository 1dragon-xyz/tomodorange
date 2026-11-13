# AGENTS.md — Guidelines for AI Coding Agents

This file defines the mandatory rules and step-by-step workflow for any AI agent or automated tool contributing to **TomodOrange**. Read this entire guide before editing any file.

## 1. Repository Architecture & Packaging Layers

Before fixing an issue, identify **which layer** the issue belongs to. Do not edit Rust crate dependencies to fix OS packaging bugs, or vice versa.

| Layer | Files | Responsibility |
| --- | --- | --- |
| **Rust Application** | `src/main.rs`, `src/audio.rs`, `src/config.rs` | `libcosmic` UI, timer logic, `rodio` audio, TOML config |
| **Cargo Dependencies** | `Cargo.toml`, `Cargo.lock` | Rust crates and locked dependency graph |
| **Task Runner & CI** | `justfile`, `.github/workflows/ci.yml` | Formatting, Clippy lints, release builds |
| **Flatpak / Flathub** | `tomodorange.yaml`, `tomodorange.metainfo.xml`, `tomodorange.desktop` | Flatpak sandbox manifest, AppStream metadata, desktop entry |
| **Arch Linux (AUR)** | `PKGBUILD` | Arch system dependencies (`depends`) and `cargo build --release --locked` |
| **Nix** | `flake.nix` | Nix package build using `Cargo.lock` |

## 2. Mandatory Contribution Workflow (Follow in Order)

1. **Confirm the root cause first.**
   - Read the relevant files and reproduce or trace the issue before making edits.
   - If the issue is in AUR packaging, Flatpak metadata, or Nix, fix that packaging layer—do not guess changes in `Cargo.toml`.
2. **Keep changes strictly scoped.**
   - Only modify files directly required for the task.
   - Do **not** downgrade or swap dependencies (e.g., `libcosmic`, `rodio`) unless explicitly required and verified to compile.
3. **Keep `Cargo.lock` in sync.**
   - If you touch `Cargo.toml`, you **must** update `Cargo.lock` and verify `cargo build --release --locked` succeeds. Both `PKGBUILD` and `flake.nix` rely on `--locked` / `Cargo.lock`.
4. **Verify all identifiers, paths, and URLs exist.**
   - Never invent placeholder usernames, unverified image/screenshot URLs, or nonexistent crate versions (e.g., `libcosmic` is sourced from git in `Cargo.toml`—do not replace it with a guessed crates.io version).
   - Keep application IDs consistent across `src/main.rs` (`APP_ID`), `src/config.rs` (`ProjectDirs`), `tomodorange.desktop`, `tomodorange.metainfo.xml`, and `tomodorange.yaml`.
5. **Run all checks before opening a PR.**
   - Never claim checks passed without actually running them and observing exit code `0`.

## 3. Mandatory Verification Commands

Every code or dependency change **must** pass all three checks from `.github/workflows/ci.yml` / `justfile`:

```bash
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo build --release --locked
```

Or using `just`:

```bash
just check
cargo build --release --locked
```

If your environment cannot compile or run these checks (e.g., missing system libraries), **explicitly state what you could not run** in the PR description instead of pretending it was tested.

## 4. Flatpak & Flathub Rules

Flathub reviewers reject submissions that look unverified, mismatched, or non-compliant. When modifying Flatpak or AppStream files (`tomodorange.yaml`, `tomodorange.metainfo.xml`, `tomodorange.desktop`):

- **Reverse-DNS App ID:** Flathub requires a valid reverse-DNS ID (e.g., `io.github.<owner>.tomodorange`) that matches the manifest filename, `<id>` in `.metainfo.xml`, `.desktop` filename, icon filename, and `APP_ID`.
- **Offline builds:** Flathub builds run offline. Never rely on Cargo fetching crates during `flatpak-builder`; vendor/generate `cargo-sources.json` (e.g., via `flatpak-cargo-generator`) and pass `--offline` / `CARGO_HOME`.
- **Real URLs & tags only:** Every `<url>`, `<screenshot>`, `git` source `url`, and `tag` must point to a real, publicly reachable URL or existing git tag in `1dragon-xyz/tomodorange`.
- **Validate metadata:** Run `appstreamcli validate --pedantic tomodorange.metainfo.xml` and test-build with `flatpak-builder` (or Flathub's `org.flatpak.Builder` linter) before submitting.
- **Minimal sandbox permissions:** Keep `finish-args` minimal (Wayland, DRI, PulseAudio/PipeWire, `--persist`) and avoid broad filesystem or X11 fallbacks unless justified.

## 5. Arch Linux (AUR) Rules

- In `PKGBUILD`, `depends` and `makedepends` are **Arch/pacman system packages only** (e.g., `libxkbcommon`, `fontconfig`, `wayland`, `alsa-lib`, `rust`, `cargo`, `cmake`, `pkg-config`).
- Never add Rust Cargo crates (such as `libcosmic` or `rodio`) to `PKGBUILD` `depends`—Cargo resolves Rust crates from `Cargo.toml` / `Cargo.lock`.

## 6. Pull Request Standards

- Use [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`, `refactor:`, `chore:`).
- Fill out `.github/PULL_REQUEST_TEMPLATE.md` completely.
- Disclose if the PR was authored or assisted by an AI agent, and list the exact commands run to verify it.
- Do not open drive-by PRs that fail CI, leave `Cargo.lock` out of sync, or solve the wrong problem.
