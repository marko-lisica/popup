use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

// Window configuration for custom mode
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

// Hardcoded window configuration for notification template
impl WindowConfig {
    pub fn notification_template() -> Self {
        Self {
            width: 500.0,
            height: 300.0,
            resizable: false,
            always_on_top: true,
            skip_taskbar: true,
            focus: true,
            visible_on_all_workspaces: true,
            closable: false,
            minimizable: false,
            hidden_title: true,
            title_bar_style: "overlay".to_string(),
        }
    }
}

// New YAML config structure - top-level keys for each subcommand
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub notification: Option<NotificationConfig>,
    #[serde(default)]
    pub custom: Option<CustomConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotificationConfig {
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
pub struct CustomConfig {
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub window: Option<WindowConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebhookConfig {
    pub url: String,
    pub payload: String,
}

// Internal config representation (used after CLI/YAML parsing)
#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub content: Content,
    pub window: WindowConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Content {
    Custom(CustomContent),
    Notification(NotificationContent),
}

#[derive(Debug, Clone, Serialize)]
pub struct CustomContent {
    pub url: String,
    pub window_title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationContent {
    pub title: String,
    pub description: String,
    pub icon: Option<String>,
    pub button_primary_text: Option<String>,
    pub button_primary_webhook: Option<WebhookConfig>,
    pub button_secondary_text: Option<String>,
    pub button_secondary_webhook: Option<WebhookConfig>,
}

// Convert from AppConfig (YAML) to internal Config
impl AppConfig {
    pub fn to_config(self) -> Result<Config, String> {
        if let Some(notification) = self.notification {
            return Ok(Config {
                content: Content::Notification(NotificationContent {
                    title: notification.title,
                    description: notification.description,
                    icon: notification.icon,
                    button_primary_text: notification.button_primary_text,
                    button_primary_webhook: notification.button_primary_webhook,
                    button_secondary_text: notification.button_secondary_text,
                    button_secondary_webhook: notification.button_secondary_webhook,
                }),
                window: WindowConfig::notification_template(),
            });
        }

        if let Some(custom) = self.custom {
            return Ok(Config {
                content: Content::Custom(CustomContent {
                    url: custom.url,
                    window_title: custom.title,
                }),
                window: custom.window.unwrap_or_default(),
            });
        }

        Err("Config must contain either 'notification' or 'custom' section".to_string())
    }
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

    let app_config: AppConfig = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML config: {}", e))?;

    app_config.to_config()
}
