use std::time::Duration;

use notify_rust::Notification;

pub struct Notificator {}

impl Notificator {
    pub fn show_notification(text: &str, timeout_ms: u64) -> Result<&str, &str> {
        let result = Notification::new()
            .appname("paradajz")
            .summary(text)
            .timeout(Duration::from_millis(timeout_ms))
            .show();

        let result = match result {
            Ok(_) => Ok("notification displayed"),
            Err(_) => Err("error displaying notification"),
        };

        result
    }
}
