mod input;
mod solutions;

use solutions::{year_2015, year_2021};
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
        "2015:1" => year_2015::day_01::run(),
        "2015:2" => year_2015::day_02::run(),
        "2015:3" => year_2015::day_03::run(),
        "2015:4" => year_2015::day_04::run(),

        "2021:15" => year_2021::day_15::run(),
        "2021:16" => year_2021::day_16::run(),
        "2021:22" => year_2021::day_22::run(),

        _ => println!("Can't run unrecognized problem '{}'", problem)
    }

    println!("--\n");
}
