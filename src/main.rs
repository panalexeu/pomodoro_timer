use std::env;
use std::process::exit;

use pomodoro_timer::{passed_args::PassedArgs, timer::Timer};


fn main() {
    let args = env::args();
    let passed_args = PassedArgs::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(-1);
    });

    let timer = Timer::new(
        passed_args.timer_type,
        passed_args.period
    );
    timer.start();
}
