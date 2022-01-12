// target area: x=57..116, y=-198..-148

#[derive(Debug)]
enum Velocity {
    Bounded(i32, usize, usize), /* (vel, start, stop) */
    Unbounded(i32, usize),      /* (vel, start) */
}

impl Velocity {
    fn overlaps(&self, other: &Self) -> bool {
        use Velocity::*;

        match self {
            Bounded(_, lstart, lstop) => match other {
                Bounded(_, rstart, rstop) => lstop >= rstart && rstop >= lstart,
                Unbounded(_, rstart) => rstart <= lstop,
            },
            Unbounded(_, lstart) => match other {
                Bounded(_, _, rstop) => lstart <= rstop,
                Unbounded(_, _) => true,
            },
        }
    }
}

fn step_to_target(velocity: i32, min: i32, max: i32, stop_at_zero: bool) -> Option<Velocity> {
    let mut pos: i32 = 0;
    let mut vel: i32 = velocity;
    let mut steps: usize = 0;
    let mut start: usize = 0;

    loop {
        steps += 1;
        pos += vel;
        if !stop_at_zero || vel > 0 {
            vel -= 1;
        }
        
        if pos >= min && pos <= max {
            if start == 0 {
                start = steps;
            }
        } else if start > 0 {
            return Some(Velocity::Bounded(velocity, start, steps - 1));
        } else if 0 < max && max < pos || 0 > min && min > pos {
            return None;
        }

        if stop_at_zero && vel == 0 {
            if start > 0 {
                return Some(Velocity::Unbounded(velocity, start));
            } else {
                return None;
            }
        }
    }
}

pub fn run() {
    let mut x_vels: Vec<Velocity> = Vec::new();
    let mut y_vels: Vec<Velocity> = Vec::new();
    let mut on_target: Vec<(&Velocity, &Velocity)> = Vec::new();

    for x in 11..117 {
        if let Some(vel) = step_to_target(x, 57, 116, true) {
            x_vels.push(vel);
        }
    }

    for y in -198..198 {
        if let Some(vel) = step_to_target(y, -198, -148, false) {
            y_vels.push(vel);
        }
    }

    for dx in x_vels.iter() {
        for dy in y_vels.iter() {
            if dx.overlaps(&dy) {
                on_target.push((dx, dy));
            }
        }
    }

    println!(" part 2: {}", on_target.len());
}