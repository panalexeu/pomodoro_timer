use std::env::Args;


pub enum TimerType {
    Pomodoro,
    ShortBreak,
    LongBreak,
}


pub struct PassedArgs {
    pub timer_type: TimerType,
    pub period: u8,
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
                    _ => return Err("Timer allowed args are: pom, brk, lgbrk")
                }
            }
            None => return Err("Timer type is not passed")
        };

        let period: u8 = match args.next() {
            Some(time) => match time.parse::<u8>() {
                Ok(res) => res,
                Err(_) => return Err("Timer allowed time is u8 values only")
            },
            None => 0
        };

        Ok(
            PassedArgs {
                timer_type,
                period,
            }
        )
    }
}