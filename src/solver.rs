use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Solver {
    type Ans1: Display;
    type Ans2: Display;

    fn solution1(&self) -> Self::Ans1;
    fn solution2(&self) -> Self::Ans2;

    fn solve(&self, day: u32) {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let a1 = self.solution1();
        let a2 = self.solution2();
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!(">>>>>> ======= Day {:2} ======= <<<<<<", day);
        println!("Answer 1: {}", a1);
        println!("Answer 2: {}", a2);
        println!(">>>>>> Time elapsed {:#?} <<<<<<", end - start);
    }
}
