# Popup - Desktop Notification Tool

A simple Tauri-based desktop application that displays a configurable popup window that stays always on top.

## Features

- Always-on-top window
- Configurable via YAML file
- Global keyboard shortcut (CMD+SHIFT+X or CTRL+SHIFT+X) to close
- Simple HTML/CSS/JS frontend (no frameworks)
- CLI-based

## Prerequisites

Before running this application, you need to install:

1. **Rust** - Visit https://www.rust-lang.org/learn/get-started#installing-rust
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** - Already installed based on npm usage

## Installation

1. Navigate to the project directory:
   ```bash
   cd popup
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

## Usage

Run the application with a config file:

```bash
npm run tauri -- dev -- config path/to/config.yaml
```

For example, with the provided example config:

```bash
npm run tauri -- dev -- config example-config.yaml
```

Or build for production:

```bash
npm run tauri -- build
```

Then run the built executable:

```bash
./src-tauri/target/release/popup config path/to/config.yaml
```

## Config File Format

Create a YAML file with the following structure:

```yaml
title: "Your Title Here"
description: "Your description text here"
```

### Example

An example config file is provided: `example-config.yaml`

```yaml
title: "Welcome to Popup!"
description: "This is a simple desktop notification app that stays always on top. Press CMD+SHIFT+X to close this window."
```

Run it with:

```bash
npm run tauri -- dev -- config example-config.yaml
```

## Keyboard Shortcuts

- **CMD+SHIFT+X** (macOS) or **CTRL+SHIFT+X** (Windows/Linux) - Close the popup window

## Project Structure

```
popup/
├── index.html           # Frontend HTML page
├── example-config.yaml  # Example configuration file
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs     # CLI argument handling
│   │   └── lib.rs      # Core logic, config loading
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
└── README.md           # This file
```

## How It Works

1. User runs `popup config path/to/config.yaml`
2. The Rust backend parses the CLI arguments using `clap`
3. The YAML config is loaded using `serde_yaml`
4. A Tauri window opens with `alwaysOnTop: true`
5. The frontend invokes the `get_config` command to retrieve config
6. The HTML page displays the title and description
7. Global shortcut CMD+SHIFT+X is registered to close the app

## Development

To run in development mode:

```bash
npm run tauri -- dev -- config example-config.yaml
```

## Building

To create a production build:

```bash
npm run tauri build
```

The executable will be in `src-tauri/target/release/`
