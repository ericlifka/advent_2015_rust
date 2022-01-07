use crate::input;

pub fn run() {
    let lines = input::read_lines("02")
        .expect("Couldn't read input");

    let mut total_paper: i32 = 0;
    let mut total_ribbon: i32 = 0;

    for line in lines {
        let mut dimensions: Vec<i32> =
            line.split("x")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

        dimensions.sort();

        total_paper += compute_paper_size(&dimensions);
        total_ribbon += compute_ribbon_length(&dimensions);
    }

    println!(" part 1: {}", total_paper);
    println!(" part 2: {}", total_ribbon);
}

fn compute_paper_size(dimensions: &Vec<i32>) -> i32 {
    let l = dimensions[ 0 ];
    let w = dimensions[ 1 ];
    let h = dimensions[ 2 ];

    3*l*w + 2*l*h + 2*w*h
}

fn compute_ribbon_length(dimensions: &Vec<i32>) -> i32 {
    let l = dimensions[ 0 ];
    let w = dimensions[ 1 ];
    let h = dimensions[ 2 ];

    2*l + 2*w + l*w*h
}