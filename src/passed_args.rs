use std::env::Args;


pub enum TimerType {
    Pomodoro,
    ShortBreak,
    LongBreak,
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
                    _ => return Err("Timer allowed args are: pom, brk, lgbrk")
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