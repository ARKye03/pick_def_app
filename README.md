# Pick Default App

A modern GTK4/Libadwaita application for managing default applications for file types (MIME types) on Linux.

## Overview

Pick Default App provides an intuitive graphical interface to view and modify which applications open specific file types on your system. It directly manages the `~/.config/mimeapps.list` file, following the [FreeDesktop MIME Applications specification](https://specifications.freedesktop.org/mime-apps-spec/mime-apps-latest.html).

## Features

- 🎨 Modern UI built with GTK4 and Libadwaita
- 🔍 Browse all MIME types configured on your system
- 📱 View available applications for each file type
- ⚙️ Set default applications for specific MIME types
- 🔄 Real-time updates to system configuration
- 🎯 Fuzzy search for quick navigation

## Prerequisites

### Runtime Dependencies
- GTK4 (>= 4.18)
- Libadwaita (>= 1.7)

### Build Dependencies
- Rust (Edition 2024)
- Cargo
- GTK4 development files
- Libadwaita development files

## Installation

### Using Nix Flakes (Recommended)

```bash
# Run directly
nix run github:arkye03/pick_def_app

# Or build and install
nix build
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/arkye03/pick_def_app.git
cd pick_def_app

# Build the project
cargo build --release

# The binary will be available at
./target/release/pick_def_app
```

## Usage

Simply run the application:

```bash
pick_def_app
```

The application will:
1. Scan your system for installed desktop applications
2. Load your current MIME type associations from `~/.config/mimeapps.list`
3. Present a user-friendly interface to view and modify these associations

## Development

Using Nix:

```bash
# Enter development shell
nix develop

# Build
cargo build

# Run
cargo run
```

## Dependencies

Key dependencies from `Cargo.toml`:
- **libadwaita** (0.8.0) - Modern GNOME UI toolkit
- **gtk4** (0.10.1) - GTK4 Rust bindings
- **freedesktop-desktop-entry** (0.7.14) - Desktop file parsing
- **fuzzy-matcher** (0.3.7) - Search functionality

## License

MIT License - Copyright (c) 2025 ARKye03

See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Note**: This application is designed for Linux systems following FreeDesktop standards. It requires a desktop environment that supports `.desktop` files and MIME type associations.
