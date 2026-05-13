# TomodOrange: AI Context Overview

This document provides architectural context for AI agents and maintainers working on TomodOrange.

## Project Vision
TomodOrange is a native COSMIC™ app designed for "Anxiety-Free Productivity." It prioritizes minimalism, visual flow, and auditory cues over complex feature sets.

## Technical Architecture
*   **Language**: Rust (2024 Edition)
*   **Framework**: `libcosmic` (based on `iced`)
*   **Window Management**: Utilizes Wayland/COSMIC APIs for "Always on Top" persistence.
*   **Audio Engine**: `rodio` with `rust-embed` for memory-buffered sound playback (Mokugyo ticks, waves).
*   **Configuration**: Standard `config.toml` (transitioning to `cosmic-config`).

## Core Components
*   `src/main.rs`: Entry point and application state management.
*   `assets/`: Embedded sounds and icons.
*   `Cargo.toml`: Dependency and build profile definitions.

## Design Tokens
*   **Work Color**: Vibrant Orange (Focus).
*   **Break Color**: Lime Green (Recovery).
*   **Typography**: Clean, oversized "Minutes-only" display.

## Maintenance Standards
*   **Formatting**: `rustfmt` is mandatory.
*   **Linting**: `clippy` must be satisfied on all PRs.
*   **Reproducibility**: All tasks must be runnable via the `justfile`.

---
*Context last updated: May 2026*
