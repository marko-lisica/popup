# Popup - Desktop Notification Tool

A simple Tauri-based desktop application that displays configurable popup windows.

## Features

- **Notification Template**: Predefined notification dialogs with buttons and webhooks
- **Custom Mode**: Display any web content with full window customization
- **YAML Configuration**: Load settings from configuration files
- Global keyboard shortcut (CMD+SHIFT+X or CTRL+SHIFT+X) to close
- CLI-based with intuitive subcommands

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

The application supports three modes via subcommands:

### 1. Notification Mode (Template)

Display a notification dialog with predefined window settings:

```bash
popup notification \
  --title "Update Available" \
  --description "A new version is ready to install" \
  --button_primary_text "Install" \
  --button_primary_webhook_url "https://webhook.site/..." \
  --button_primary_webhook_payload '{"action":"install"}'
```

### 2. Custom Mode

Display custom web content with full window control:

```bash
popup custom \
  --url "https://example.com" \
  --title "My App" \
  --width 800 \
  --height 600 \
  --resizable true \
  --closable true \
  --hide_title_bar true
```

**Available flags:**
- `--hide_title_bar` - Hide the title bar (works on macOS, Windows, and Linux)
- `--visible` - Start window visible immediately (default: false)
- `--transparent` - Enable transparent window for better performance (default: true)
- Plus all the standard window flags: `width`, `height`, `resizable`, `closable`, etc.

### 3. File Mode

Load configuration from a YAML file:

```bash
popup file --path example-notification.yaml
```

Or with short flag:

```bash
popup file -p example-custom.yaml
```

## Config File Format

### Notification Template

Create a YAML file with a `notification` section:

```yaml
notification:
  title: "Update Available"
  description: "A new version is ready to install."
  icon: "path/to/icon.png"
  button_primary_text: "Install"
  button_primary_webhook:
    url: "https://webhook.site/your-webhook-id"
    payload: '{"action":"install","version":"2.0.0"}'
  button_secondary_text: "Later"
  button_secondary_webhook:
    url: "https://webhook.site/your-webhook-id"
    payload: '{"action":"dismiss"}'
```

Note: Window settings are predefined for notification templates (500x300, non-resizable).

### Custom Mode

Create a YAML file with a `custom` section:

```yaml
custom:
  url: "https://example.com"
  title: "My Custom App"
  window:
    width: 800
    height: 600
    resizable: true
    always_on_top: false
    skip_taskbar: false
    focus: true
    visible_on_all_workspaces: false
    closable: true
    minimizable: true
    hide_title_bar: false
    visible: false
    transparent: true
```

Example files are provided:
- `example-notification.yaml` - Notification template example
- `example-custom.yaml` - Custom mode example

## Keyboard Shortcuts

- **CMD+SHIFT+X** (macOS) or **CTRL+SHIFT+X** (Windows/Linux) - Close the popup window

## Project Structure

```
popup/
├── index.html                    # Frontend HTML page
├── example-notification.yaml     # Example notification config
├── example-custom.yaml          # Example custom config
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── main.rs              # CLI subcommands and argument handling
│   │   └── lib.rs               # Core logic, config structures, YAML loading
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
└── README.md                    # This file
```

## How It Works

1. User runs a subcommand: `popup notification ...`, `popup custom ...`, or `popup file -p config.yaml`
2. The Rust backend parses CLI arguments using `clap` with subcommands
3. Config is built from CLI flags or loaded from YAML file
4. A Tauri window is created with specified settings
5. For notifications, the React frontend loads and displays the notification UI
6. For custom mode, the specified URL is loaded directly
7. Global shortcut CMD+SHIFT+X is registered to close the app

## Development

To run in development mode, use one of the subcommands:

```bash
# Notification mode
npm run tauri dev -- notification --title "Test" --description "Testing"

# Custom mode
npm run tauri dev -- custom --url "https://example.com"

# File mode
npm run tauri dev -- file -p example-notification.yaml
```

## Building

To create a production build:

```bash
npm run tauri build
```

The executables will be in `src-tauri/target/release/`

Run the built executable:

```bash
# Notification
./src-tauri/target/release/popup notification --title "Update" --description "New version available"

# Custom
./src-tauri/target/release/popup custom --url "https://example.com" --width 800 --height 600

# File
./src-tauri/target/release/popup file -p config.yaml
```
