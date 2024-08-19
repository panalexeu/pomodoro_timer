use std::env;
use std::process::exit;
use pomodoro_timer::{PassedArgs, Timer};

fn main() {
    let mut args = env::args();
    let passed_args = PassedArgs::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(-1);
    });

    let timer = Timer::new(passed_args.timer_type);
    timer.start();
}
