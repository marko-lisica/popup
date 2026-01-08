// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use std::sync::Mutex;
use tauri::State;

#[derive(Parser, Debug)]
#[command(name = "popup")]
#[command(about = "A simple popup window tool", long_about = None)]
struct Args {
    /// Ignore Tauri dev flags
    #[arg(long, hide = true, global = true)]
    no_default_features: bool,

    #[arg(long, hide = true, global = true)]
    color: Option<String>,

    /// Path to the YAML config file. You can define windows settings and layout in the config file.
    #[arg(long)]
    config: Option<String>,

    /// Content type: 'webview' or 'notification'
    #[arg(long, value_name = "TYPE")]
    r#type: Option<String>,

    /// URL to load for webview type
    #[arg(long)]
    url: Option<String>,

    /// Title (for notification or webview window title)
    #[arg(long)]
    title: Option<String>,

    /// Description (for notification type)
    #[arg(long)]
    description: Option<String>,

    /// Icon URL or file path (for notification type)
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

    /// DEPRECATED: Use --type webview --url instead
    #[arg(long)]
    webview: Option<String>,

    /// Window width in pixels.
    #[arg(long)]
    width: Option<f64>,

    /// Window height in pixels.
    #[arg(long)]
    height: Option<f64>,

    /// Controls whether the window can be resized.
    #[arg(long)]
    resizable: Option<bool>,

    /// Keep window always on top of other windows.
    #[arg(long)]
    always_on_top: Option<bool>,

    /// Hide from taskbar/dock.
    #[arg(long)]
    skip_taskbar: Option<bool>,

    /// Automatically focus window when opened.
    #[arg(long)]
    focus: Option<bool>,

    /// Show on all virtual desktops (macOS only).
    #[arg(long)]
    visible_on_all_workspaces: Option<bool>,

    /// Controls whether window shows the close button.
    #[arg(long)]
    closable: Option<bool>,

    /// Controls whether window shows the minimize button.
    #[arg(long)]
    minimizable: Option<bool>,

    /// Hide the title bar.
    #[arg(long)]
    hidden_title: Option<bool>,

    /// Title bar style. Can be one of "overlay", "transparent", or "visible".
    #[arg(long)]
    title_bar_style: Option<String>,

    /// List available content types
    #[arg(long)]
    templates: bool,
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

fn main() {
    let args = Args::parse();

    // Handle --templates flag
    if args.templates {
        println!("Available content types:");
        println!();
        println!("  webview       Load external webpage or local HTML file");
        println!("                Example: popup --type webview --url https://example.com");
        println!();
        println!("  notification  Display a notification dialog with buttons");
        println!("                Example: popup --type notification --title 'Update' --description 'Please update'");
        println!();
        println!("Window settings can be customized via --config YAML file or CLI flags.");
        println!("Run 'popup --help' for all available options.");
        std::process::exit(0);
    }

    // Validate webview URL if provided
    if let Some(ref url) = args.webview {
        if !url.starts_with("http://")
            && !url.starts_with("https://")
            && !url.starts_with("file://")
        {
            eprintln!("Error: --webview URL must start with http://, https://, or file://");
            eprintln!("Examples:");
            eprintln!("  --webview https://example.com");
            eprintln!("  --webview http://localhost:8080");
            eprintln!("  --webview file:///path/to/page.html");
            std::process::exit(1);
        }
    }

    // Validate that at least config or type is provided
    if args.config.is_none() && args.r#type.is_none() && args.webview.is_none() {
        eprintln!("Error: Must provide --config, --type, or --webview");
        eprintln!("Examples:");
        eprintln!("  popup --config example-config.yaml");
        eprintln!("  popup --type notification --title 'Update!' --description 'Please update'");
        eprintln!("  popup --type webview --url https://example.com");
        eprintln!("  popup --webview https://example.com  (deprecated)");
        std::process::exit(1);
    }

    // Load config if provided, otherwise use defaults
    let config = match args.config {
        Some(path) => match popup_lib::load_config(&path) {
            Ok(config) => {
                println!("Loaded config successfully");
                config
            }
            Err(e) => {
                eprintln!("Error loading config: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            // No config file, build from CLI flags
            println!("No config file provided, using CLI flags");

            // Determine content type
            let content_type = args.r#type.as_deref().or(if args.webview.is_some() {
                Some("webview")
            } else {
                None
            });

            let content = match content_type {
                Some("webview") => {
                    let url = args
                        .url
                        .clone()
                        .or(args.webview.clone())
                        .ok_or_else(|| {
                            eprintln!("Error: --url is required for webview type");
                            std::process::exit(1);
                        })
                        .unwrap();

                    popup_lib::Content::Webview(popup_lib::WebviewContent {
                        url,
                        window_title: args.title.clone(),
                    })
                }
                Some("notification") => {
                    let title = args
                        .title
                        .clone()
                        .ok_or_else(|| {
                            eprintln!("Error: --title is required for notification type");
                            std::process::exit(1);
                        })
                        .unwrap();

                    let description = args
                        .description
                        .clone()
                        .ok_or_else(|| {
                            eprintln!("Error: --description is required for notification type");
                            std::process::exit(1);
                        })
                        .unwrap();

                    // Build webhooks if URLs are provided
                    let button_primary_webhook = match (
                        args.button_primary_webhook_url.clone(),
                        args.button_primary_webhook_payload.clone(),
                    ) {
                        (Some(url), Some(payload)) => {
                            Some(popup_lib::WebhookConfig { url, payload })
                        }
                        (Some(_), None) => {
                            eprintln!("Error: --button-primary-webhook-payload required when --button-primary-webhook-url is provided");
                            std::process::exit(1);
                        }
                        _ => None,
                    };

                    let button_secondary_webhook = match (
                        args.button_secondary_webhook_url.clone(),
                        args.button_secondary_webhook_payload.clone(),
                    ) {
                        (Some(url), Some(payload)) => {
                            Some(popup_lib::WebhookConfig { url, payload })
                        }
                        (Some(_), None) => {
                            eprintln!("Error: --button-secondary-webhook-payload required when --button-secondary-webhook-url is provided");
                            std::process::exit(1);
                        }
                        _ => None,
                    };

                    popup_lib::Content::Notification(popup_lib::NotificationContent {
                        title,
                        description,
                        icon: args.icon.clone(),
                        button_primary_text: args.button_primary_text.clone(),
                        button_primary_webhook,
                        button_secondary_text: args.button_secondary_text.clone(),
                        button_secondary_webhook,
                    })
                }
                Some(other) => {
                    eprintln!(
                        "Error: Unknown type '{}'. Must be 'webview' or 'notification'",
                        other
                    );
                    std::process::exit(1);
                }
                None => {
                    eprintln!("Error: --type is required when not using --config");
                    std::process::exit(1);
                }
            };

            popup_lib::Config {
                content,
                window: None,
            }
        }
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
        .invoke_handler(tauri::generate_handler![get_config, exit_with_code])
        .setup(move |app| {
            // Set macOS activation policy to hide from dock (Accessory allows windows)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Get window config from YAML or use defaults, then merge with CLI overrides
            let mut window_config = config_clone
                .window
                .as_ref()
                .cloned()
                .unwrap_or_default()
                .merge_with_cli_overrides(
                    args.width,
                    args.height,
                    args.resizable,
                    args.always_on_top,
                    args.skip_taskbar,
                    args.focus,
                    args.visible_on_all_workspaces,
                    args.closable,
                    args.minimizable,
                    args.hidden_title,
                    args.title_bar_style,
                );

            // Determine webview URL and window title based on content type
            let (webview_url, window_title) = match &config_clone.content {
                popup_lib::Content::Webview(webview) => {
                    // Load external URL directly
                    let url =
                        tauri::WebviewUrl::External(webview.url.parse().unwrap_or_else(|e| {
                            eprintln!("Error: Failed to parse webview URL: {}", e);
                            std::process::exit(1);
                        }));
                    let title = webview
                        .window_title
                        .clone()
                        .unwrap_or_else(|| "Popup".to_string());
                    (url, title)
                }
                popup_lib::Content::Notification(notification) => {
                    // Apply notification template overrides (template wins)
                    window_config.width = 500.0;
                    window_config.height = 300.0;
                    window_config.resizable = false;
                    window_config.skip_taskbar = true;

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
                .minimizable(window_config.minimizable);

            // Apply title bar style
            let window_builder = if window_config.hidden_title {
                window_builder.hidden_title(true)
            } else {
                window_builder
            };

            let window_builder = match window_config.title_bar_style.as_str() {
                "overlay" => window_builder.title_bar_style(tauri::TitleBarStyle::Overlay),
                "transparent" => window_builder.title_bar_style(tauri::TitleBarStyle::Transparent),
                "visible" => window_builder.title_bar_style(tauri::TitleBarStyle::Visible),
                _ => {
                    eprintln!(
                        "Unknown title bar style: {}, using overlay",
                        window_config.title_bar_style
                    );
                    window_builder.title_bar_style(tauri::TitleBarStyle::Overlay)
                }
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
