use std::thread::sleep;
use std::time::{Duration};

use chrono::Local;

use super::passed_args::TimerType;


pub struct Timer {
    timer_type: TimerType,
    period: u8, // time period in minutes
}

impl Timer {
    pub fn new(timer_type: TimerType) -> Self {
        match timer_type {
            TimerType::Pomodoro => Timer { timer_type, period: 25 },
            TimerType::ShortBreak => Timer { timer_type, period: 5 },
            TimerType::LongBreak => Timer { timer_type, period: 10 },
        }
    }

    pub fn start(&self) {
        let period_in_sec = self.period as u64 * 60;
        let duration = Duration::from_secs(period_in_sec);

        self.start_timer_msg();
        let start_time = Local::now().format("%H:%M:%S");
        println!("Start time: {start_time}");

        sleep(duration);

        let end_time = Local::now().format("%H:%M:%S");
        println!("End time: {end_time}");
    }

    fn start_timer_msg(&self) {
        let timer_name = match &self.timer_type {
            TimerType::Pomodoro => format!("[Pomodoro Time] {}", '\u{1F345}'),
            TimerType::ShortBreak => format!("[Short Break] {}", '\u{1F369}'),
            TimerType::LongBreak => format!("[Long Break] {}", '\u{1F389}')
        };

        let logo_part = "=".repeat(timer_name.chars().count() + 1 + 6);
        //                                                            ^^^^^ - + 1 because emoji
        // takes 2 characters to be displayed in terminal but counts as one char in rust, + 6 to
        // align logo fancy

        println!("{}", timer_name.chars().count());
        println!("{}", logo_part);
        println!(" | {timer_name} |");
        println!("{}", logo_part);
        println!();
    }
}