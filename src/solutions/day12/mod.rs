use crate::solver::Solver;
use std::cmp::Ordering;
use std::collections::HashSet;

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
        moons.find_cycle()
    }
}

struct Moon {
    positions: [i64; 3],
    velocity: [i64; 3],
}
impl Moon {
    fn new(pos: &Vec<i64>) -> Moon {
        Moon {
            positions: [pos[0], pos[1], pos[2]],
            velocity: [0, 0, 0],
        }
    }
}

struct Moons {
    list: Vec<Moon>,
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
                        .map(|pos_str| pos_str.trim()[2..].parse::<i64>().unwrap())
                        .collect();
                    Moon::new(&positions)
                })
                .collect(),
        }
    }

    fn find_cycle(&mut self) -> usize {
        let x = self.find_axis_cycle(0);
        let y = self.find_axis_cycle(1);
        let z = self.find_axis_cycle(2);

        lcm(lcm(x, y), z)
    }

    fn find_axis_cycle(&mut self, axis: usize) -> usize {
        let mut states = HashSet::new();
        let mut step: usize = 0;
        loop {
            let mut state = Vec::with_capacity(self.list.len() * 2);
            for moon in &self.list {
                state.push(moon.positions[axis]);
                state.push(moon.velocity[axis]);
            }
            let state = state.into_iter().collect::<Vec<_>>();
            if !states.insert(state) {
                return step;
            }

            self.step_axis(axis);
            step += 1;
        }
    }

    fn step_axis(&mut self, axis: usize) {
        let len = self.list.len();
        for i in 0..len {
            for j in 0..len {
                if i != j {
                    self.list[i].velocity[axis] += calc_axis_velocity(
                        self.list[i].positions[axis],
                        self.list[j].positions[axis],
                    );
                }
            }
        }

        for moon in &mut self.list {
            moon.positions[axis] += moon.velocity[axis];
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
                moons[i].velocity[0] +=
                    calc_axis_velocity(moons[i].positions[0], moons[j].positions[0]);
                moons[i].velocity[1] +=
                    calc_axis_velocity(moons[i].positions[1], moons[j].positions[1]);
                moons[i].velocity[2] +=
                    calc_axis_velocity(moons[i].positions[2], moons[j].positions[2]);
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
}

fn calc_axis_velocity(v1: i64, v2: i64) -> i64 {
    let diff = v1 - v2;
    match diff.cmp(&0) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let gcd = {
        let mut x = a;
        let mut y = b;
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        x
    };
    a * (b / gcd)
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
        assert_eq!(2772, m.find_cycle());
    }

    #[test]
    fn test_universe_loop_repeated_after_4686774924() {
        let mut m = Moons::new(INPUT2);
        assert_eq!(4686774924, m.find_cycle());
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
