# New config structure plan

Currently you can add config file or webview. I want to implement that you can add webview as part of config. Besides the webview, I want to change config file structure.

So the config file will now have 2 main top level keys: `content` and `window` (rename `windows` to `window`). Still in YAML format.

## Example config file:

### Webview type

```yaml
content:
    type: webview
    url: https://example.com
    window_title: Example webview #only if titleBarStyle is defined
window:
    width: 800
    height: 600
```

### Notification type

If user specifies content.type as `notification`, the content will have fields that are in example below, and following `window` fields will be ignored: `width`, `height`, `titleBarStyle`, `resizable`, `skipTaskbar`.

```yaml
content:
    type: notification
    title: Update your OS!
    description: Please update your OS to the latest version.
    icon: https://example.com/icon.png #optiona
    button_primary: Update Now #default "Ok"
    button_secondary: Remind me later #default "Cancel"
window:
    width: 800 #ignored, set to 500
    height: 600 #ignored, set to 300
    
```
