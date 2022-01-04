mod solutions;
use solutions::{day_01};

use std::{env, time::Instant};


fn main() {
    let start = Instant::now();

    let mut days: Vec<String> = env::args().collect();
    days.remove(0);

    for day in days {
        run_day(&day);
    }

    println!("Total time: {}ms", start.elapsed().as_millis())
}

fn run_day(day: &str) {
    match day {
        "1" => day_01::run(),
        _ => println!("Can't run unrecognized day '{}'", day)
    }
}
