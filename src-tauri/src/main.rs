// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, Subcommand};
use std::sync::Mutex;
use tauri::State;

#[derive(Parser, Debug)]
#[command(name = "popup")]
#[command(about = "A simple popup window tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Ignore Tauri dev flags
    #[arg(long, hide = true, global = true)]
    no_default_features: bool,

    #[arg(long, hide = true, global = true)]
    color: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display a notification dialog (predefined template with fixed window settings)
    Notification {
        /// Notification title
        #[arg(long)]
        title: String,

        /// Notification description
        #[arg(long)]
        description: String,

        /// Icon URL or file path
        #[arg(long)]
        icon: Option<String>,

        /// Primary button text (default: "Ok")
        #[arg(long)]
        button_primary_text: Option<String>,

        /// Primary button webhook URL
        #[arg(long)]
        button_primary_webhook_url: Option<String>,

        /// Primary button webhook payload (JSON string)
        #[arg(long)]
        button_primary_webhook_payload: Option<String>,

        /// Secondary button text (default: "Cancel")
        #[arg(long)]
        button_secondary_text: Option<String>,

        /// Secondary button webhook URL
        #[arg(long)]
        button_secondary_webhook_url: Option<String>,

        /// Secondary button webhook payload (JSON string)
        #[arg(long)]
        button_secondary_webhook_payload: Option<String>,
    },

    /// Display custom web content with full window control
    Custom {
        /// URL to load (http://, https://, or file://)
        #[arg(long)]
        url: String,

        /// Window title
        #[arg(long)]
        title: Option<String>,

        /// Window width in pixels
        #[arg(long)]
        width: Option<f64>,

        /// Window height in pixels
        #[arg(long)]
        height: Option<f64>,

        /// Controls whether the window can be resized
        #[arg(long)]
        resizable: Option<bool>,

        /// Keep window always on top of other windows
        #[arg(long)]
        always_on_top: Option<bool>,

        /// Hide from taskbar/dock
        #[arg(long)]
        skip_taskbar: Option<bool>,

        /// Automatically focus window when opened
        #[arg(long)]
        focus: Option<bool>,

        /// Show on all virtual desktops (macOS only)
        #[arg(long)]
        visible_on_all_workspaces: Option<bool>,

        /// Controls whether window shows the close button
        #[arg(long)]
        closable: Option<bool>,

        /// Controls whether window shows the minimize button
        #[arg(long)]
        minimizable: Option<bool>,

        /// Hide the title bar (works cross-platform)
        #[arg(long)]
        hide_title_bar: Option<bool>,

        /// Start window visible (default: false)
        #[arg(long)]
        visible: Option<bool>,

        /// Enable transparent window (default: true)
        #[arg(long)]
        transparent: Option<bool>,
    },

    /// Load configuration from a YAML file
    File {
        /// Path to the YAML config file
        #[arg(long, short = 'p')]
        path: String,
    },
}

#[tauri::command]
fn get_config(state: State<popup_lib::AppState>) -> Result<popup_lib::Config, String> {
    let config = state.config.lock().unwrap();
    match config.as_ref() {
        Some(cfg) => Ok(cfg.clone()),
        None => Err("No config loaded".to_string()),
    }
}

#[tauri::command]
fn exit_with_code(code: i32) {
    std::process::exit(code);
}

#[tauri::command]
async fn resize_window_to_content(
    window: tauri::Window,
    width: f64,
    height: f64,
) -> Result<(), String> {
    use tauri::Size;

    // Use provided width and height
    // Use Size::Logical to set the inner content size (excludes window decorations)
    window
        .set_size(Size::Logical(tauri::LogicalSize::new(width, height)))
        .map_err(|e| format!("Failed to resize window: {}", e))
}

fn main() {
    let cli = Cli::parse();

    // Build config based on subcommand
    let config = match cli.command {
        Commands::Notification {
            title,
            description,
            icon,
            button_primary_text,
            button_primary_webhook_url,
            button_primary_webhook_payload,
            button_secondary_text,
            button_secondary_webhook_url,
            button_secondary_webhook_payload,
        } => {
            // Build webhooks if URLs are provided
            let button_primary_webhook = match (
                button_primary_webhook_url,
                button_primary_webhook_payload,
            ) {
                (Some(url), Some(payload)) => Some(popup_lib::WebhookConfig { url, payload }),
                (Some(_), None) => {
                    eprintln!("Error: --button-primary-webhook-payload required when --button-primary-webhook-url is provided");
                    std::process::exit(1);
                }
                _ => None,
            };

            let button_secondary_webhook = match (
                button_secondary_webhook_url,
                button_secondary_webhook_payload,
            ) {
                (Some(url), Some(payload)) => Some(popup_lib::WebhookConfig { url, payload }),
                (Some(_), None) => {
                    eprintln!("Error: --button-secondary-webhook-payload required when --button-secondary-webhook-url is provided");
                    std::process::exit(1);
                }
                _ => None,
            };

            popup_lib::Config {
                content: popup_lib::Content::Notification(popup_lib::NotificationContent {
                    title,
                    description,
                    icon,
                    button_primary_text,
                    button_primary_webhook,
                    button_secondary_text,
                    button_secondary_webhook,
                }),
                window: popup_lib::WindowConfig::notification_template(),
            }
        }
        Commands::Custom {
            url,
            title,
            width,
            height,
            resizable,
            always_on_top,
            skip_taskbar,
            focus,
            visible_on_all_workspaces,
            closable,
            minimizable,
            hide_title_bar,
            visible,
            transparent: _,
        } => {
            // Validate URL format
            if !url.starts_with("http://")
                && !url.starts_with("https://")
                && !url.starts_with("file://")
            {
                eprintln!("Error: URL must start with http://, https://, or file://");
                eprintln!("Examples:");
                eprintln!("  --url https://example.com");
                eprintln!("  --url http://localhost:8080");
                eprintln!("  --url file:///path/to/page.html");
                std::process::exit(1);
            }

            // Build window config from CLI flags or use defaults
            let mut window = popup_lib::WindowConfig::default();
            if let Some(v) = width {
                window.width = v;
            }
            if let Some(v) = height {
                window.height = v;
            }
            if let Some(v) = resizable {
                window.resizable = v;
            }
            if let Some(v) = always_on_top {
                window.always_on_top = v;
            }
            if let Some(v) = skip_taskbar {
                window.skip_taskbar = v;
            }
            if let Some(v) = focus {
                window.focus = v;
            }
            if let Some(v) = visible_on_all_workspaces {
                window.visible_on_all_workspaces = v;
            }
            if let Some(v) = closable {
                window.closable = v;
            }
            if let Some(v) = minimizable {
                window.minimizable = v;
            }
            if let Some(v) = hide_title_bar {
                window.hide_title_bar = v;
            }
            if let Some(v) = visible {
                window.visible = v;
            }
            // Note: transparent flag is kept in config for future use but not applied to window builder

            popup_lib::Config {
                content: popup_lib::Content::Custom(popup_lib::CustomContent {
                    url,
                    window_title: title,
                }),
                window,
            }
        }
        Commands::File { path } => match popup_lib::load_config(&path) {
            Ok(config) => {
                println!("Loaded config from: {}", path);
                config
            }
            Err(e) => {
                eprintln!("Error loading config: {}", e);
                std::process::exit(1);
            }
        },
    };

    // Build and run the application
    let shortcut_plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut("CmdOrCtrl+Shift+X")
        .unwrap_or_else(|e| {
            eprintln!("Failed to register global shortcut: {}", e);
            std::process::exit(1);
        })
        .with_handler(|app, _shortcut, _event| {
            app.exit(0);
        })
        .build();

    let config_clone = config.clone();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(shortcut_plugin)
        .manage(popup_lib::AppState {
            config: Mutex::new(Some(config.clone())),
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            exit_with_code,
            resize_window_to_content
        ])
        .setup(move |app| {
            // Set macOS activation policy to hide from dock (Accessory allows windows)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window_config = &config_clone.window;

            // Determine webview URL and window title based on content type
            let (webview_url, window_title) = match &config_clone.content {
                popup_lib::Content::Custom(custom) => {
                    // Load external URL directly
                    let url = tauri::WebviewUrl::External(custom.url.parse().unwrap_or_else(|e| {
                        eprintln!("Error: Failed to parse URL: {}", e);
                        std::process::exit(1);
                    }));
                    let title = custom
                        .window_title
                        .clone()
                        .unwrap_or_else(|| "Popup".to_string());
                    (url, title)
                }
                popup_lib::Content::Notification(notification) => {
                    // Load React app for notification UI
                    (tauri::WebviewUrl::default(), notification.title.clone())
                }
            };

            // Create the window programmatically with config values
            let window_builder = tauri::WebviewWindowBuilder::new(app, "main", webview_url)
                .title(&window_title)
                .inner_size(window_config.width, window_config.height)
                .resizable(window_config.resizable)
                .always_on_top(window_config.always_on_top)
                .skip_taskbar(window_config.skip_taskbar)
                .focused(window_config.focus)
                .visible_on_all_workspaces(window_config.visible_on_all_workspaces)
                .closable(window_config.closable)
                .minimizable(window_config.minimizable)
                .visible(window_config.visible);

            // Apply title bar hiding (platform-specific behavior)
            #[cfg(target_os = "macos")]
            let window_builder = if window_config.hide_title_bar {
                // On macOS: use hidden_title + overlay style for clean look
                window_builder
                    .hidden_title(true)
                    .title_bar_style(tauri::TitleBarStyle::Overlay)
            } else {
                window_builder
            };

            #[cfg(target_os = "windows")]
            let window_builder = if window_config.hide_title_bar {
                window_builder.decorations(false)
            } else {
                window_builder.decorations(true)
            };

            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            let window_builder = if window_config.hide_title_bar {
                window_builder.decorations(false)
            } else {
                window_builder.decorations(true)
            };

            window_builder.build().map_err(|e| {
                eprintln!("Failed to create window: {}", e);
                e
            })?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
