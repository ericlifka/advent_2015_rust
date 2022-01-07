mod input;
mod solutions;

use solutions::year_2015;
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
    println!("\n-- Day {}", day);

    // TODO: come up with something more dynamic for this
    match day {
        "2015:1" => year_2015::day_01::run(),
        "2015:2" => year_2015::day_02::run(),
        "2015:3" => year_2015::day_03::run(),
        "2015:4" => year_2015::day_04::run(),
        _ => println!("Can't run unrecognized day '{}'", day)
    }

    println!("--\n");
}
