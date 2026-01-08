# Window Configuration Guide

The popup application now supports three ways to configure window properties, with the following priority:

**Priority Order (highest to lowest):**
1. **CLI Flags** - Override everything
2. **YAML Config File** - Override defaults
3. **Default Values** - Used when nothing else is specified

## Configuration Options

All window properties support these three configuration methods:

- `width` (default: 800) - Window width in pixels
- `height` (default: 600) - Window height in pixels
- `resizable` (default: false) - Whether window can be resized
- `alwaysOnTop` (default: true) - Keep window on top
- `skipTaskbar` (default: true) - Hide from taskbar/dock
- `focus` (default: true) - Focus window on creation
- `visibleOnAllWorkspaces` (default: true) - Show on all virtual desktops
- `closable` (default: false) - Whether window has close button
- `minimizable` (default: false) - Whether window can be minimized
- `hiddenTitle` (default: true) - Hide the title bar
- `titleBarStyle` (default: "overlay") - Options: "Overlay", "Transparent", "Visible"

## Usage Examples

### 1. Using Defaults Only
Create a minimal config without windows section:
```yaml
title: "My Popup"
description: "Uses all default window settings"
```

```bash
popup --config minimal.yaml
```

### 2. Using YAML Configuration
```yaml
title: "Custom Window"
description: "With custom window settings"

windows:
  width: 1000
  height: 800
  resizable: true
  closable: true
```

```bash
popup --config custom.yaml
```

### 3. Using CLI Flags (Override YAML)
```bash
# Override specific properties via CLI
popup --config example-config.yaml --width 1200 --height 900

# Make window resizable and closable
popup --config example-config.yaml --resizable true --closable true

# Change title bar style
popup --config example-config.yaml --title-bar-style Transparent

# Mix YAML and CLI (CLI wins)
popup --config custom.yaml --width 500 --always-on-top false
```

### 4. Partial YAML Configuration
You can specify only the properties you want to override in YAML:

```yaml
title: "Partial Config"
description: "Only override width and height"

windows:
  width: 1024
  height: 768
  # All other properties use defaults
```

## Note on CLI Flag Names

CLI flags use kebab-case (hyphens):
- YAML: `alwaysOnTop` → CLI: `--always-on-top`
- YAML: `skipTaskbar` → CLI: `--skip-taskbar`
- YAML: `visibleOnAllWorkspaces` → CLI: `--visible-on-all-workspaces`
- YAML: `hiddenTitle` → CLI: `--hidden-title`
- YAML: `titleBarStyle` → CLI: `--title-bar-style`
