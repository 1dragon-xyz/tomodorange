# TomodOrange Justfile

binary_name := "tomodorange"
desktop_file := "tomodorange.desktop"

# Build the application in release mode
build:
    cargo build --release

# Run the application
run:
    cargo run --release

# Check for lints and formatting
check:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings

# Install dependencies (Pop!_OS / Ubuntu / Debian)
install-deps:
    sudo apt-get update
    sudo apt-get install -y \
        cmake \
        pkg-config \
        libwayland-dev \
        libxkbcommon-dev \
        libfontconfig1-dev \
        libdbus-1-dev \
        libasound2-dev \
        libexpat1-dev \
        libfreetype-dev

# Install the application locally
install: build
    mkdir -p ~/.local/bin
    mkdir -p ~/.local/share/applications
    mkdir -p ~/.local/share/icons/hicolor/256x256/apps
    cp target/release/{{binary_name}} ~/.local/bin/{{binary_name}}
    cp assets/icon.png ~/.local/share/icons/hicolor/256x256/apps/{{binary_name}}.png
    cp {{desktop_file}} ~/.local/share/applications/
    update-desktop-database ~/.local/share/applications/

# Uninstall the application
uninstall:
    rm -f ~/.local/bin/{{binary_name}}
    rm -f ~/.local/share/applications/{{desktop_file}}
    rm -f ~/.local/share/icons/hicolor/256x256/apps/{{binary_name}}.png
    update-desktop-database ~/.local/share/applications/

# Clean build artifacts
clean:
    cargo clean
