use crate::solver::Solver;
use std::string::String;

mod input;

pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> Self::Ans1 {
        0
    }
    
    fn solution2(&self) -> Self::Ans2 {
        0
    }
}


struct Moon {
    positions: [i32; 3],
    velocity: [i32; 3],
}
impl Moon {
    fn new(pos: &Vec<i32>) -> Moon {
        Moon {
            positions: [ pos[0], pos[1], pos[2] ],
            velocity: [0, 0, 0]
        }
    }
}

fn parse_scan_data(scan_data: &str) -> Vec<Moon> {
    scan_data
        .lines()
        .skip(1)
        .map(|moon_data| {
            let positions: Vec<i32> = moon_data[1..moon_data.len() - 1]
                .split(',')
                .map(|pos_str| {
                    pos_str.trim()[2..].parse::<i32>().unwrap()
                })
                .collect();
            Moon::new(&positions)
        })
        .collect()
}


#[cfg(test)]
mod test {
    use crate::solutions::day12::*;

    #[test]
    fn test_gravity_apply() {
        let moons = parse_scan_data(INPUT1);
    }

//10 steps, 179 energy
const INPUT1: &str = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

// 100 steps, 1940
const INPUT2: &str = "
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";
}

