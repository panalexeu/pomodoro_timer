use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

use chrono::Local;
use rodio::{Decoder, OutputStream, Sink};

use super::passed_args::TimerType;


pub struct Timer {
    timer_type: TimerType,
    period: u8, // time period in minutes
}

impl Timer {
    pub fn new(timer_type: TimerType, period: u8) -> Self {
        Timer {
            period: match timer_type {
                TimerType::Pomodoro =>  if period != 0 { period } else { 25 },
                TimerType::ShortBreak => if period != 0 { period } else { 5 },
                TimerType::LongBreak => if period != 0 { period } else { 10 },
            },
            timer_type
        }
    }

    pub fn start(&self) {
        let period_in_sec = self.period as u64 * 1;
        let duration = Duration::from_secs(period_in_sec);

        self.start_timer_msg();
        self.play_notification();
        let start_time = Local::now().format("%H:%M:%S");
        println!("Start time: {start_time}");

        sleep(duration);

        let end_time = Local::now().format("%H:%M:%S");
        println!("End time: {end_time}");

        self.play_notification();
    }

    fn start_timer_msg(&self) {
        let timer_name = match self.timer_type {
            TimerType::Pomodoro => format!("[Pomodoro Time] {}", '\u{1F345}'),
            TimerType::ShortBreak => format!("[Short Break] {}", '\u{1F369}'),
            TimerType::LongBreak => format!("[Long Break] {}", '\u{1F389}')
        };

        let logo_part = "=".repeat(timer_name.chars().count() + 1 + 6);
        //                                                               ^^^^^ - + 1 because emoji
        // takes 2 characters to be displayed in terminal but counts as one char in rust, + 6 to
        // align logo fancy

        println!("{}", logo_part);
        println!(" | {timer_name} |");
        println!("{}\n", logo_part);
    }

    fn play_notification(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open("sounds/beach_notification.wav").unwrap());
        let source = Decoder::new(file).unwrap();

        sink.set_volume(2.0);
        sink.append(source);
        sink.sleep_until_end();
    }
}
