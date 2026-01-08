use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WindowConfig {
    #[serde(default = "default_width")]
    pub width: f64,
    #[serde(default = "default_height")]
    pub height: f64,
    #[serde(default = "default_resizable")]
    pub resizable: bool,
    #[serde(default = "default_always_on_top")]
    pub always_on_top: bool,
    #[serde(default = "default_skip_taskbar")]
    pub skip_taskbar: bool,
    #[serde(default = "default_focus")]
    pub focus: bool,
    #[serde(default = "default_visible_on_all_workspaces")]
    pub visible_on_all_workspaces: bool,
    #[serde(default = "default_closable")]
    pub closable: bool,
    #[serde(default = "default_minimizable")]
    pub minimizable: bool,
    #[serde(default = "default_hidden_title")]
    pub hidden_title: bool,
    #[serde(default = "default_title_bar_style")]
    pub title_bar_style: String,
}

fn default_width() -> f64 {
    800.0
}
fn default_height() -> f64 {
    600.0
}
fn default_resizable() -> bool {
    false
}
fn default_always_on_top() -> bool {
    true
}
fn default_skip_taskbar() -> bool {
    true
}
fn default_focus() -> bool {
    true
}
fn default_visible_on_all_workspaces() -> bool {
    true
}
fn default_closable() -> bool {
    false
}
fn default_minimizable() -> bool {
    false
}
fn default_hidden_title() -> bool {
    true
}
fn default_title_bar_style() -> String {
    "overlay".to_string()
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            resizable: default_resizable(),
            always_on_top: default_always_on_top(),
            skip_taskbar: default_skip_taskbar(),
            focus: default_focus(),
            visible_on_all_workspaces: default_visible_on_all_workspaces(),
            closable: default_closable(),
            minimizable: default_minimizable(),
            hidden_title: default_hidden_title(),
            title_bar_style: default_title_bar_style(),
        }
    }
}

impl WindowConfig {
    /// Merge CLI overrides into this config. CLI values take precedence.
    pub fn merge_with_cli_overrides(
        mut self,
        width: Option<f64>,
        height: Option<f64>,
        resizable: Option<bool>,
        always_on_top: Option<bool>,
        skip_taskbar: Option<bool>,
        focus: Option<bool>,
        visible_on_all_workspaces: Option<bool>,
        closable: Option<bool>,
        minimizable: Option<bool>,
        hidden_title: Option<bool>,
        title_bar_style: Option<String>,
    ) -> Self {
        if let Some(v) = width {
            self.width = v;
        }
        if let Some(v) = height {
            self.height = v;
        }
        if let Some(v) = resizable {
            self.resizable = v;
        }
        if let Some(v) = always_on_top {
            self.always_on_top = v;
        }
        if let Some(v) = skip_taskbar {
            self.skip_taskbar = v;
        }
        if let Some(v) = focus {
            self.focus = v;
        }
        if let Some(v) = visible_on_all_workspaces {
            self.visible_on_all_workspaces = v;
        }
        if let Some(v) = closable {
            self.closable = v;
        }
        if let Some(v) = minimizable {
            self.minimizable = v;
        }
        if let Some(v) = hidden_title {
            self.hidden_title = v;
        }
        if let Some(v) = title_bar_style {
            self.title_bar_style = v;
        }
        self
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub content: Content,
    #[serde(default)]
    pub window: Option<WindowConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Content {
    Webview(WebviewContent),
    Notification(NotificationContent),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebviewContent {
    pub url: String,
    #[serde(default)]
    pub window_title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotificationContent {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub button_primary_text: Option<String>,
    #[serde(default)]
    pub button_primary_webhook: Option<WebhookConfig>,
    #[serde(default)]
    pub button_secondary_text: Option<String>,
    #[serde(default)]
    pub button_secondary_webhook: Option<WebhookConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebhookConfig {
    pub url: String,
    pub payload: String,
}

pub struct AppState {
    pub config: Mutex<Option<Config>>,
}

pub fn load_config(path: &str) -> Result<Config, String> {
    // Expand to absolute path
    let expanded_path = if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        path.replacen("~", &home, 1)
    } else if !path.starts_with("/") {
        // If relative path, make it relative to current working directory
        let cwd = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        cwd.join(path).to_str().ok_or("Invalid path")?.to_string()
    } else {
        path.to_string()
    };

    let content = fs::read_to_string(&expanded_path)
        .map_err(|e| format!("Failed to read config file '{}': {}", expanded_path, e))?;

    let config: Config = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML config: {}", e))?;

    Ok(config)
}
