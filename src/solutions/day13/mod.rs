use crate::solver::Solver;
use crate::intcode::*;
use std::collections::HashSet;

mod input;

pub struct Problem;
impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        let mut game: Game = Game::new(input::INPUT);
        game.run_for_tiles();
        game.count_tiles(TileID::BLOCK)
    }

    fn solution2(&self) -> Self::Ans2 {
        let mut game: Game = Game::new(input::INPUT);
        game.play()
    }
}

#[derive(Eq, PartialEq, Hash)]
enum TileID {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL
}

impl From<i64> for TileID {
    fn from(val: i64) -> TileID {
        match val {
            0 => TileID::EMPTY,
            1 => TileID::WALL,
            2 => TileID::BLOCK,
            3 => TileID::PADDLE,
            4 => TileID::BALL,
            _ => panic!("Unknown ID for tile: {}", val)
        }
    }
}

impl From<TileID> for i64 {
    fn from(id: TileID) -> i64 {
        match id {
            TileID::EMPTY => 0,
            TileID::WALL => 1,
            TileID::BLOCK => 2,
            TileID::PADDLE => 3,
            TileID::BALL => 4,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Tile {
    x: i64,
    y: i64,
    id: TileID
}
impl Tile {
    fn new(x: i64, y: i64, id: TileID) -> Tile {
        Tile { x: x, y: y, id: id}
    }
}

struct Game {
    cpu: Cpu,
    screen: HashSet<Tile>,
    paddle_x: i64,
    ball_x: i64,
    score: i64
}
impl Game {
    fn new(program: &str) -> Game {
        Game {
            cpu: Cpu::new(program),
            screen: HashSet::new(),
            paddle_x: 0,
            ball_x : 0,
            score: 0
        }
    }

    fn run_for_tiles(&mut self) {
        let mut out_cnt: u8 = 0;
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        loop {
            match self.cpu.step() {
                CpuStatus::Output(out) => {
                    match out_cnt {
                        0 => { x = out; out_cnt += 1; },
                        1 => { y = out; out_cnt += 1; },
                        2 => {
                            self.screen.insert(Tile::new(x, y, out.into()));
                            out_cnt = 0; 
                        },
                        _ => panic!("Invalid out cnt: {}", out_cnt)
                    }
                },
                CpuStatus::Running => continue,
                CpuStatus::Finished => break,
                CpuStatus::WaitForInput => unreachable!()
            }
        }
    }

    fn play(&mut self) -> usize {
        let mut out_cnt: u8 = 0;
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        self.cpu.code[0] = 2;
        loop {
            match self.cpu.step() {
                CpuStatus::Output(out) => {
                    match out_cnt {
                        0 => { x = out; out_cnt += 1; },
                        1 => { y = out; out_cnt += 1; },
                        2 => {
                            if x == -1 && y == 0 {
                                self.score = out;
                                out_cnt = 0;
                            }
                            else {
                                self.screen.insert(Tile::new(x, y, out.into()));
                                if TileID::BALL == out.into() {
                                    self.ball_x = x;
                                }
                                else if TileID::PADDLE == out.into() {
                                    self.paddle_x = x;
                                }

                                out_cnt = 0; 
                            }
                        },
                        _ => panic!("Invalid out cnt: {}", out_cnt)
                    }
                },
                CpuStatus::Running => continue,
                CpuStatus::Finished => break,
                CpuStatus::WaitForInput => {
                    let input: i64;
                    if self.ball_x < self.paddle_x {
                        input = -1;
                    }
                    else if self.ball_x > self.paddle_x {
                        input = 1;
                    }
                    else {
                        input = 0;
                    }
                    self.cpu.push_input(input);
                }
            }
        }
        self.score as usize
    }

    fn count_tiles(&self, id: TileID) -> usize {
        self.screen
            .iter()
            .filter(|t| t.id == id)
            .count()
    }
}

