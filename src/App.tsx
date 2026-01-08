import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import NotificationContent from "./templates/notification/NotificationContent";

interface WebhookConfig {
  url: string;
  payload: string;
}

interface NotificationContentData {
  type: "notification";
  title: string;
  description: string;
  icon?: string;
  button_primary_text?: string;
  button_primary_webhook?: WebhookConfig;
  button_secondary_text?: string;
  button_secondary_webhook?: WebhookConfig;
}

interface WebviewContentData {
  type: "webview";
  url: string;
  window_title?: string;
}

type ContentData = NotificationContentData | WebviewContentData;

interface Config {
  content: ContentData;
  window?: {
    width?: number;
    height?: number;
  };
}

function App() {
  const [config, setConfig] = useState<Config | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadConfig() {
      try {
        const loadedConfig = await invoke<Config>("get_config");
        setConfig(loadedConfig);
      } catch (err) {
        setError(String(err));
      }
    }

    loadConfig();
  }, []);

  if (error) {
    return (
      <div className="error">
        <h1>Error</h1>
        <p>Failed to load config: {error}</p>
      </div>
    );
  }

  if (!config) {
    return <div style={{ padding: "20px" }}>Loading...</div>;
  }

  // Route based on content type
  if (config.content.type === "notification") {
    return <NotificationContent content={config.content} />;
  }

  if (config.content.type === "webview") {
    return (
      <div className="error">
        <h1>Error</h1>
        <p>Webview content should be loaded directly, not through React app</p>
      </div>
    );
  }

  return (
    <div className="error">
      <h1>Error</h1>
      <p>Unknown content type</p>
    </div>
  );
}

export default App;
