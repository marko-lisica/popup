import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Button } from "@base-ui/react/button";
import { useEffect, useRef } from "react";
import "./NotificationContent.css";

interface WebhookConfig {
  url: string;
  payload: string;
}

interface NotificationContentProps {
  content: {
    title: string;
    description: string;
    icon?: string;
    button_primary_text?: string;
    button_primary_webhook?: WebhookConfig;
    button_secondary_text?: string;
    button_secondary_webhook?: WebhookConfig;
  };
}

const EXIT_PRIMARY_BUTTON = 0;
const EXIT_SECONDARY_BUTTON = 2;

async function sendWebhook(webhook: WebhookConfig): Promise<void> {
  try {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 10000);

    const response = await fetch(webhook.url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: webhook.payload,
      signal: controller.signal,
    });

    clearTimeout(timeoutId);

    if (response.ok) {
      console.log("Webhook sent successfully");
    } else {
      console.error(`Webhook failed with status: ${response.status}`);
    }
  } catch (error) {
    if (error instanceof Error && error.name === "AbortError") {
      console.error("Webhook timeout after 10 seconds");
    } else {
      console.error("Webhook error:", error);
    }
  }
}

async function handleButtonClick(
  exitCode: number,
  webhook?: WebhookConfig,
): Promise<void> {
  console.log(`Button clicked, exit code: ${exitCode}`);

  if (webhook) {
    console.log("Sending webhook...");
    await sendWebhook(webhook);
  }

  console.log("Invoking exit_with_code...");
  try {
    await invoke("exit_with_code", { code: exitCode });
  } catch (error) {
    console.error("Failed to exit:", error);
  }
}

function getIconSrc(icon: string): string {
  if (icon.startsWith("http://") || icon.startsWith("https://")) {
    return icon;
  }
  // For local files, return as-is for now
  return icon;
}

function NotificationContent({ content }: NotificationContentProps) {
  const contentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const measureAndResize = async () => {
      if (!contentRef.current) return;

      // Wait for layout to complete
      await new Promise((resolve) => setTimeout(resolve, 50));

      // Measure the full document height - this includes all content even if clipped
      const bodyScrollHeight = document.body.scrollHeight;
      const htmlScrollHeight = document.documentElement.scrollHeight;
      const bodyOffsetHeight = document.body.offsetHeight;

      // Use the largest measurement
      const contentHeight = Math.max(
        bodyScrollHeight,
        htmlScrollHeight,
        bodyOffsetHeight,
      );

      // Minimum 200px
      const finalHeight = Math.max(contentHeight, 200);

      try {
        // Get current window width to preserve it
        const window = getCurrentWindow();
        const size = await window.innerSize();
        const scaleFactor = await window.scaleFactor();
        const currentWidth = size.width / scaleFactor;

        await invoke("resize_window_to_content", {
          width: currentWidth,
          height: finalHeight,
        });
      } catch (error) {
        console.error("Failed to resize window:", error);
      }
    };

    // Use ResizeObserver to detect content changes
    const observer = new ResizeObserver(() => {
      measureAndResize();
    });

    if (contentRef.current) {
      observer.observe(contentRef.current);
    }

    // Initial measurement with slight delay to ensure DOM is ready
    requestAnimationFrame(() => {
      measureAndResize();
    });

    return () => observer.disconnect();
  }, [content]);

  return (
    <div className="notification-card" ref={contentRef}>
      <div className="notification-body">
        <div className="notification-content">
          {content.icon && (
            <img
              src={getIconSrc(content.icon)}
              alt="Notification icon"
              className="notification-icon"
            />
          )}
          <h1 className="notification-title">{content.title}</h1>
          <div className="notification-text">
            <p className="notification-description">{content.description}</p>
          </div>
        </div>
      </div>

      <div className="notification-footer">
        <div className="notification-buttons">
          {content.button_secondary_text && (
            <Button
              className="notification-button notification-button-secondary"
              onClick={() =>
                handleButtonClick(
                  EXIT_SECONDARY_BUTTON,
                  content.button_secondary_webhook,
                )
              }
            >
              {content.button_secondary_text}
            </Button>
          )}
          <Button
            className="notification-button notification-button-primary"
            onClick={() =>
              handleButtonClick(
                EXIT_PRIMARY_BUTTON,
                content.button_primary_webhook,
              )
            }
          >
            {content.button_primary_text || "Done"}
          </Button>
        </div>
      </div>
    </div>
  );
}

export default NotificationContent;
