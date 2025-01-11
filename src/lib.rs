#![doc = include_str!("../README.md")]

use std::io::{IsTerminal, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Create a progress printer with the total number of steps and a prefix message
///
/// [`print`](Printer::print) is used to print updates to the progress,
/// and the progress is done printing when dropping the printer.
pub fn printer(total: usize, prefix: impl std::fmt::Display) -> Printer {
    Printer::new(total, prefix)
}

#[derive(Debug)]
pub struct Printer {
    /// Cached terminal width to truncate long messages, 0 means do not truncate
    term_width: usize,
    /// Total number of steps in the progress, 0 means unknown and total will not be printed
    total: usize,
    /// Prefix to print in the message
    prefix: String,
    /// Min interval between 2 prints
    throttle_duration: Duration,
    /// Internal states used to throttle printing
    throttle_current_count: AtomicUsize,
    /// Internal states used to throttle printing
    /// Max count is calculated based on the speed of the progress
    throttle_max_count: AtomicUsize,
    /// Start time used to calculate speed
    start_time: Instant,
}

impl Printer {
    /// See [`printer`]
    pub fn new(total: usize, prefix: impl std::fmt::Display) -> Self {
        let term_width = if std::io::stderr().is_terminal() {
            match terminal_size::terminal_size() {
                Some((width, _)) => width.0 as usize,
                None => 0,
            }
        } else {
            0
        };

        let prefix = prefix.to_string();
        let throttle_duration = Duration::from_millis(50);

        Self {
            term_width,
            total,
            prefix,
            throttle_duration,
            throttle_current_count: AtomicUsize::new(0),
            throttle_max_count: AtomicUsize::new(0),
            start_time: Instant::now(),
        }
    }

    /// Set a minimum interval between 2 prints
    pub fn set_throttle_duration(&mut self, duration: Duration) {
        self.throttle_duration = duration;
    }

    /// Update the progress with the current step
    ///
    /// This is equivalent to calling `print` with empty string
    pub fn update(&self, current: usize) {
        self.print(current, "");
    }

    /// Print the progress with the current step and a message
    pub fn print(&self, current: usize, text: impl std::fmt::Display) {
        let throttle_max_count = self.throttle_max_count.load(Ordering::Relaxed);
        if throttle_max_count == 0 {
            // no speed info yet, use time
            let elapsed = self.start_time.elapsed();
            if elapsed < self.throttle_duration {
                return;
            }
            self.throttle_max_count
                .store(current + 1, Ordering::Relaxed);
        } else {
            let throttle_current_count = self.throttle_current_count.load(Ordering::Relaxed);
            if throttle_current_count < throttle_max_count {
                self.throttle_current_count.fetch_add(1, Ordering::Relaxed);
                return;
            }
            self.throttle_current_count.store(0, Ordering::Relaxed);
        }

        let prefix = if self.total == 0 {
            format!("{1} {0} ", self.prefix, current)
        } else {
            let mut s = format!("[{1}/{2}] {0}: ", self.prefix, current, self.total);
            let elapsed = self.start_time.elapsed().as_secs_f32();
            if elapsed > 2.0 {
                let percentage = format!("{:.02}% ", (current as f32 / self.total as f32) * 100.0);
                let speed = current as f32 / elapsed; // items/second
                                                      // update throttling based on speed
                let throttle_max_count = (self.throttle_duration.as_secs_f32() * speed) as usize;
                self.throttle_max_count
                    .store(throttle_max_count, Ordering::Relaxed);
                let eta = format!("ETA {:.02}s ", (self.total - current) as f32 / speed);
                s.push_str(&percentage);
                s.push_str(&eta);
            }
            s
        };
        let prefix_len = prefix.len();
        let prefix = if prefix_len > self.term_width {
            &prefix[prefix_len - self.term_width + 1..]
        } else {
            &prefix
        };
        let remaining = self.term_width - prefix_len - 1;
        let text = text.to_string();
        let text_len = text.len();
        let text = if text_len > remaining {
            let start = text_len - remaining;
            &text[start..]
        } else {
            &text
        };
        eprint!("\r{}{}\u{1b}[0K", prefix, text);
        let _ = std::io::stderr().flush();
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        if self.total == 0 {
            println!("\u{1b}[1K\r{}", self.prefix);
        } else {
            println!("\u{1b}[1K\r[{1}/{1}] {0}", self.prefix, self.total);
        }
    }
}
