use std::fmt::Display;

pub trait Solver {
    type Ans1: Display;
    type Ans2: Display;

    fn solution1(&self) -> Self::Ans1;
    fn solution2(&self) -> Self::Ans2;

    fn solve(&self, day: u32) {
        let a1 = self.solution1();
        let a2 = self.solution2();
        println!(">>>>>> ======= Day {:2} ======= <<<<<<", day);
        println!("Answer 1: {}", a1);
        println!("Answer 2: {}", a2);
    }
}
