use std::env::Args;
use std::thread::sleep;
use std::time::Duration;

pub enum TimerType {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

pub struct Timer {
    timer_type: TimerType,
    period: u8 // time in minutes
}

impl Timer {
    pub fn new(timer_type: TimerType) -> Timer {
        match timer_type {
            TimerType::Pomodoro => Timer { timer_type, period: 25 },
            TimerType::ShortBreak => Timer { timer_type, period: 5 },
            TimerType::LongBreak => Timer { timer_type, period: 10 },
        }
    }

    pub fn start(&self) {
        let duration_in_sec = *&self.period as u64 * 1;
        let duration = Duration::from_secs(duration_in_sec);
        sleep(duration);
        println!("It was a wasted minute :(");
    }
}

pub struct PassedArgs {
    pub timer_type: TimerType,
    period: u8,
}

impl PassedArgs {
    pub fn new(mut args: Args) -> Result<PassedArgs, &'static str> {
        args.next();

        let timer_type: TimerType = match args.next() {
            Some(timer) => {
                match timer.as_str() {
                    "pom" => TimerType::Pomodoro,
                    "brk" => TimerType::ShortBreak,
                    "lgbrk" => TimerType::LongBreak,
                    _ => return Err("Only following timer arguments are allowed:\n * pom;\
                                    \n * brk;\n * lgbrk;")
                }
            }
            None => return Err("Timer type is not passed")
        };

        Ok(
            PassedArgs {
                timer_type,
                period: 255,
            }
        )
    }
}