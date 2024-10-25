use crate::solver::Solver;

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


#[cfg(test)]
mod test {
    use crate::solutions::day12::*;

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

