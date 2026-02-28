use crate::APP_NAME;
use anyhow::{Context, Result, anyhow};
use libnotify::Notification;

pub struct Notifier {
    enabled: bool,
    template: String,
}

impl Notifier {
    pub fn new(enabled: bool, template: String) -> Result<Self> {
        if enabled {
            libnotify::init(APP_NAME)
                .map_err(|e| anyhow!("Failed to initialize libnotify: {e}"))?;
        }
        Ok(Self { enabled, template })
    }

    pub fn notify(&self, device_name: &str, action: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let body = render_body(&self.template, device_name, action);
        let notification = Notification::new(APP_NAME, Some(body.as_str()), None);
        notification
            .show()
            .with_context(|| "Failed to show notification")
    }
}

impl Drop for Notifier {
    fn drop(&mut self) {
        if self.enabled {
            libnotify::uninit();
        }
    }
}

fn render_body(template: &str, device_name: &str, action: &str) -> String {
    template
        .replace("{{device_name}}", device_name)
        .replace("{{action}}", action)
}

#[cfg_attr(test, allow(clippy::panic))]
#[cfg(test)]
mod tests {
    use super::render_body;

    #[test]
    fn renders_template_placeholders() {
        let template = "Device {{device_name}} {{action}}.";
        let rendered = render_body(template, "USB Drive", "connected");
        assert_eq!(rendered, "Device USB Drive connected.");
    }
}
