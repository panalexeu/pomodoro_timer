use std::thread::sleep;
use std::time::{Duration};

use super::passed_args::TimerType;


pub struct Timer {
    timer_type: TimerType,
    period: u8, // time period in minutes
}

impl Timer {
    pub fn new(timer_type: TimerType) -> Timer {
        match timer_type {
            TimerType::Pomodoro => Timer { timer_type, period: 25 },
            TimerType::ShortBreak => Timer { timer_type, period: 1 },
            TimerType::LongBreak => Timer { timer_type, period: 10 },
        }
    }

    pub fn start(&self) {
        let period_in_sec = *&self.period as u64 * 60;
        let duration = Duration::from_secs(period_in_sec);
        sleep(duration);

        println!("the time has come!");
     }
}