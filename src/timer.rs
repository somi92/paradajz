use chrono::{DateTime, Duration, Local, Timelike};

/// Represents a timer.
pub struct Timer {
    duration_ms: i64,
    remaining_ms: i64,
    elapsed_ms: i64,
    paused: bool,
    terminated: bool,
    finish_at: DateTime<Local>,
    on_expired: fn() -> (),
}

impl Timer {
    pub fn new(duration_ms: i64, on_expired: fn() -> ()) -> Self {
        let now = Local::now();
        let finish_at = now + Duration::milliseconds(duration_ms);
        let elapsed_ms = 0;
        let remaining_ms = duration_ms - elapsed_ms;
        Timer {
            duration_ms,
            elapsed_ms,
            remaining_ms,
            finish_at,
            paused: false,
            terminated: false,
            on_expired,
        }
    }

    pub fn reset(&mut self, duration_ms: i64) {
        let now = Local::now();
        self.duration_ms = duration_ms;
        self.finish_at = now + Duration::milliseconds(duration_ms);
        self.elapsed_ms = 0;
        self.remaining_ms = self.duration_ms - self.elapsed_ms;
        self.paused = false;
        self.terminated = false;
    }

    pub fn tick(&mut self) {
        if self.is_expired() {
            return;
        }

        let now = Local::now();
        if self.paused {
            self.finish_at = now + Duration::milliseconds(self.remaining_ms);
        } else {
            self.remaining_ms = self.finish_at.timestamp_millis() - now.timestamp_millis();
            self.elapsed_ms = self.duration_ms - self.remaining_ms;
        }

        if self.is_expired() {
            (self.on_expired)();
        }
    }

    pub fn is_expired(&self) -> bool {
        self.remaining_ms <= 0
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn terminate(&mut self) {
        self.terminated = true;
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn terminated(&self) -> bool {
        self.terminated
    }

    pub fn elapsed_ratio(&self) -> f64 {
        let ratio = self.elapsed_ms as f64 / self.duration_ms as f64;
        if ratio > 1.0 {
            return 1.0;
        }
        ratio
    }

    pub fn remaining_time_formatted(&self) -> String {
        let remaining = Duration::milliseconds(self.remaining_ms);

        let hours = remaining.num_hours();
        let minutes = remaining.num_minutes() % 60;
        let seconds = remaining.num_seconds() % 60;

        if hours == 0 {
            return format!("{:02}:{:02}", minutes, seconds);
        }

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn finish_time_formatted(&self) -> String {
        let hours = self.finish_at.hour();
        let minutes = self.finish_at.minute();
        let seconds = self.finish_at.second();

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
