# TomodOrange for COSMIC™

**Stay fresh, stay focused. A native Pomodoro timer for the COSMIC™ desktop.**

![TomodOrange Logo](assets/icon.png)

![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg) ![Rust](https://img.shields.io/badge/rust-2024-orange) ![Platform](https://img.shields.io/badge/platform-COSMIC-purple)

## The TomodOrange Standard

TomodOrange isn't just another timer. It is built on a "Lean-First" philosophy to ensure your focus is never broken by the tools meant to help you.

*   **🔒 Total Privacy**: Built with a **Zero-Network Policy**. TomodOrange does not "phone home," track your usage, or require an account. Your focus data never leaves your machine.
*   **⚡ Maximum Efficiency**: Forget heavy web-based apps. TomodOrange is built in **Native Rust**, making it lightning-fast and extremely battery-friendly. It’s practically invisible to your system resources.
*   **🌊 Flow-State Audio**: Features rhythmic **Mokugyo ticking** and high-fidelity ambient loops scientifically chosen to induce deep concentration and prevent "clock-watching" anxiety.
*   **🎨 True COSMIC Integration**: Not a port or a wrapper. This is a **native libcosmic application** that respects your system themes, window behaviors, and visual language perfectly.
*   **🛠 Set-and-Forget Reliability**: From the hardened Flatpak sandbox to reproducible builds, TomodOrange is engineered for stability. Install it once, and it will work forever.

## Features

*   **Native COSMIC App**: Built with `libcosmic` for seamless integration.
*   **Always-on-Top**: The timer stays visible above other windows (configurable).
*   **Smart Transitions**: Automatically cycles between Work and Break sessions.
*   **Audio Feedback**: 
    *   **Tick**: A rhythmic sound during work sessions.
    *   **Gong**: Signals the transition between states.
    *   **Ambient Loops**: Relaxing sounds during break time.
*   **Persistent Configuration**: Saves your preferences in `~/.config/tomodorange/config.toml`.

## Installation

### Flatpak (Recommended)
TomodOrange is available on **Flathub**. You can install it directly from the **COSMIC AppCenter** or via command line:
```bash
flatpak install flathub tomodorange
```

### Arch Linux (AUR)
Install using your favorite AUR helper:
```bash
yay -S tomodorange
```

### Nix
Add TomodOrange to your `flake.nix` or run directly:
```bash
nix run github:1dragon-xyz/tomodorange
```

## Configuration

Settings are stored in `~/.config/tomodorange/config.toml`. You can adjust:
*   `work_minutes`: Duration of focus sessions.
*   `break_minutes`: Duration of break sessions.
*   `always_on_top`: Whether the widget stays above other windows.
*   `sound_enabled`: Enable or disable all audio cues (ticks, gongs, break loops).

## Other Installation Methods

### Build from Source
If you prefer to build the app yourself, follow these steps:

#### Prerequisites
*   **OS**: Pop!_OS 24.04+ or any Linux distribution with the COSMIC™ desktop.
*   **Toolchain**: Rust (2024 edition) and `just` command runner.

#### Setup
1.  **Clone the repository**:
    ```bash
    git clone https://github.com/1dragon-xyz/tomodorange.git
    cd tomodorange
    ```

2.  **Install system dependencies** (Pop!_OS/Ubuntu):
    ```bash
    just install-deps
    ```

3.  **Build and Run**:
    ```bash
    just run
    ```

4.  **Install to System Menu**:
    ```bash
    just install
    ```

## Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get started.

## License

TomodOrange is dual-licensed under the [MIT](LICENSE) and [Apache-2.0](LICENSE) licenses.
