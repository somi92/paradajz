use std::time::Duration;

use notify_rust::Notification;

pub struct Notificator {}

impl Notificator {
    pub fn show_notification(text: &str, timeout_ms: u64) {
        let result = Notification::new()
            .appname("paradajz")
            .summary(text)
            .timeout(Duration::from_millis(timeout_ms))
            .show();

        match result {
            Ok(_) => println!("notification displayed"),
            Err(_) => println!("error displaying notification"),
        }
    }
}
