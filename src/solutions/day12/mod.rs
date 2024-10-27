use crate::solver::Solver;
use std::fmt;

mod input;

pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> Self::Ans1 {
        let _ = Moons::new(input::INPUT);
        0
    }
    
    fn solution2(&self) -> Self::Ans2 {
        0
    }
}


struct Moon {
    positions: [i32; 3],
    velocity: [i32; 3],
    id: usize,
}
impl Moon {
    fn new(pos: &Vec<i32>, id: usize) -> Moon {
        Moon {
            positions: [ pos[0], pos[1], pos[2] ],
            velocity: [0, 0, 0],
            id: id,
        }
    }
}

struct Moons {
    list: Vec<Moon>,
}

impl fmt::Display for Moons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for m in &self.list {
            writeln!(f, "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}. y={:3}, z={:3}>",
                m.positions[0], m.positions[1], m.positions[2],
                m.velocity[0], m.velocity[1], m.velocity[2]);
        }

        Ok(())
    }
}

impl Moons {
    fn new(scan_data: &str) -> Moons {
        Moons {
            list: scan_data
                .lines()
                .skip(1)
                .enumerate()
                .map(|(idx, moon_data)| {
                    let positions: Vec<i32> = moon_data[1..moon_data.len() - 1]
                        .split(',')
                        .map(|pos_str| {
                            pos_str.trim()[2..].parse::<i32>().unwrap()
                        })
                        .collect();
                    Moon::new(&positions, idx)
                })
                .collect()
        }
    }

    fn apply_gravity(&mut self) {
        for moon in &self.list {
            let x_lower: i32 = 0;
            let y_lower: i32 = 0;
            let z_lower: i32 = 0;
            for cmp_moon in &self.list {
                if moon.id == cmp_moon.id {
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::day12::*;

    #[test]
    fn test_gravity_apply() {
        let m = Moons::new(INPUT1);
        println!("{}", m);
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

