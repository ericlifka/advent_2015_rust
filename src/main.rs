mod input;
mod intcode;
mod solutions;

use solutions::{year_2015, year_2019, year_2021};
use std::{env, time::Instant};


fn main() {
    let start = Instant::now();

    let mut problems: Vec<String> = env::args().collect();
    problems.remove(0);

    for problem in problems {
        run_problem(&problem);
    }

    println!("Total time: {}ms", start.elapsed().as_millis())
}

fn run_problem(problem: &str) {
    println!("\n-- Problem {}", problem);

    // TODO: come up with something more dynamic for this
    match problem {
        "2015:1"  => year_2015::day_01::run(),
        "2015:2"  => year_2015::day_02::run(),
        "2015:3"  => year_2015::day_03::run(),
        "2015:4"  => year_2015::day_04::run(),
        "2015:6"  => year_2015::day_06::run(),

        "2019:2"  => year_2019::day_02::run(),
        "2019:5"  => year_2019::day_05::run(),

        "2021:15" => year_2021::day_15::run(),
        "2021:16" => year_2021::day_16::run(),
        "2021:17" => year_2021::day_17::run(),
        "2021:19" => year_2021::day_19::run(),
        "2021:22" => year_2021::day_22::run(),
        "2021:23" => year_2021::day_23::run(),
        "2021:24" => year_2021::day_24::run(),
        "2021:25" => year_2021::day_25::run(),

        _ => println!("Can't run unrecognized problem '{}'", problem)
    }

    println!("--\n");
}
