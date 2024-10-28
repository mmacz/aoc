use crate::solver::Solver;
use std::fmt;
use std::cmp::Ordering;

mod input;

pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        let mut moons = Moons::new(input::INPUT);
        moons.step_n(1000);
        moons.energy()
    }
    
    fn solution2(&self) -> Self::Ans2 {
        let mut moons = Moons::new(input::INPUT);
        let initial_pos = moons.copy_pos();
        let mut loop_cnt: usize = 0;

        loop {
            moons.step_n(1);
            loop_cnt = loop_cnt + 1;

            let universe_loop = moons.copy_pos();
            let len = initial_pos.len();
            for i in 0..len {
                if  (initial_pos[i][0] == universe_loop[i][0])
                    && (initial_pos[i][1] == universe_loop[i][1])
                    && (initial_pos[i][2] == universe_loop[i][2]) 
                {
                    return loop_cnt;
                }
            }
        }
    }
}


struct Moon {
    positions: [i64; 3],
    velocity: [i64; 3],
}
impl Moon {
    fn new(pos: &Vec<i64>) -> Moon {
        Moon {
            positions: [ pos[0], pos[1], pos[2] ],
            velocity: [0, 0, 0],
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
                m.velocity[0], m.velocity[1], m.velocity[2])?;
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
                .map(|moon_data| {
                    let positions: Vec<i64> = moon_data[1..moon_data.len() - 1]
                        .split(',')
                        .map(|pos_str| {
                            pos_str.trim()[2..].parse::<i64>().unwrap()
                        })
                        .collect();
                    Moon::new(&positions)
                })
                .collect()
        }
    }


    fn apply_gravity(&mut self) {
        let len = self.list.len();
        for i in 0..len {
            let moons = &mut self.list;
            for j in 0..len {
                if i == j {
                    continue;
                }
                moons[i].velocity[0] += calc_axis_velocity(moons[i].positions[0], moons[j].positions[0]);
                moons[i].velocity[1] += calc_axis_velocity(moons[i].positions[1], moons[j].positions[1]);
                moons[i].velocity[2] += calc_axis_velocity(moons[i].positions[2], moons[j].positions[2]);
            }
        }
    }

    fn apply_velocity(&mut self) {
        for m in &mut self.list {
            m.positions[0] += m.velocity[0];
            m.positions[1] += m.velocity[1];
            m.positions[2] += m.velocity[2];
        }
    }

    fn step_n(&mut self, steps: usize) {
        for _ in 0..steps {
            self.apply_gravity();
            self.apply_velocity();
        }
    }

    fn energy(&self) -> i64 {
        self.list
            .iter()
            .map(|moon| {
                let pot: i64 = moon.positions.iter().map(|p| p.abs()).sum();
                let kin: i64 = moon.velocity.iter().map(|v| v.abs()).sum();
                pot * kin
            })
            .sum()
    }

    fn copy_pos(&self) -> Vec<[i64; 3]> {
        let mut pos: Vec<[i64; 3]> = Vec::new();

        for m in &self.list {
            pos.push(m.positions.clone());
        }

        pos
    }
}

fn calc_axis_velocity(v1: i64, v2: i64) -> i64 {
    let diff = v1 - v2;
    match diff.cmp(&0) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::day12::*;

    #[test]
    fn test_energy_after_10_steps() {
        let mut m = Moons::new(INPUT1);
        m.step_n(10);
        assert_eq!(179, m.energy());
    }

    #[test]
    fn test_energy_after_100_steps() {
        let mut m = Moons::new(INPUT2);
        m.step_n(100);
        assert_eq!(1940, m.energy());
    }

    #[test]
    fn test_universe_loop_repeated_after_2772() {
        let mut m = Moons::new(INPUT1);
        let initial_pos = m.copy_pos();

        m.step_n(2772);
        let universe_loop = m.copy_pos();

        let len = initial_pos.len();
        for i in 0..len {
            assert_eq!(initial_pos[i][0], universe_loop[i][0], "Invalid positions for {} moon", i);
            assert_eq!(initial_pos[i][1], universe_loop[i][1], "Invalid positions for {} moon", i);
            assert_eq!(initial_pos[i][2], universe_loop[i][2], "Invalid positions for {} moon", i);
        }
    }

    #[test]
    fn test_universe_loop_repeated_after_4686774924() {
        let mut m = Moons::new(INPUT2);
        let initial_pos = m.copy_pos();

        m.step_n(4686774924);
        let universe_loop = m.copy_pos();

        let len = initial_pos.len();
        for i in 0..len {
            assert_eq!(initial_pos[i][0], universe_loop[i][0], "Invalid positions for {} moon", i);
            assert_eq!(initial_pos[i][1], universe_loop[i][1], "Invalid positions for {} moon", i);
            assert_eq!(initial_pos[i][2], universe_loop[i][2], "Invalid positions for {} moon", i);
        }
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

