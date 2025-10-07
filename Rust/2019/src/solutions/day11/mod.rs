use crate::intcode::*;
use crate::solver::Solver;
use std::collections::HashMap;
use std::fmt;

mod input;

pub struct Problem;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = ImgVec;

    fn solution1(&self) -> Self::Ans1 {
        let mut robot: Robot = Robot::new();
        robot.paint(Color::BLACK).len()
    }

    fn solution2(&self) -> Self::Ans2 {
        let mut robot: Robot = Robot::new();
        ImgVec(robot.paint(Color::WHITE))
    }
}

pub struct ImgVec(HashMap<Position, Color>);
impl ImgVec {
    fn to_char_vec(&self) -> Vec<char> {
        if self.0.is_empty() {
            return Vec::new();
        }

        let (x_max, x_min) = self.get_max_min_axis(0);
        let (y_max, y_min) = self.get_max_min_axis(1);

        let mut result = Vec::new();
        for y in (y_min..=y_max).rev() {
            for x in x_min..=x_max {
                let pos = Position::new(x, y);
                let ch = match self.0.get(&pos).unwrap_or(&Color::BLACK) {
                    Color::BLACK => ' ',
                    Color::WHITE => '#',
                };
                result.push(ch);
            }
        }
        result
    }

    fn get_max_min_axis(&self, axis: i32) -> (i64, i64) {
        let positions: Vec<i64> = self
            .0
            .keys()
            .map(|pos| match axis {
                0 => pos.x,
                1 => pos.y,
                _ => panic!("Invalid axis: {}", axis),
            })
            .collect();

        let max = *positions.iter().max().unwrap_or(&0);
        let min = *positions.iter().min().unwrap_or(&0);

        (max, min)
    }

    fn get_width(&self) -> i64 {
        let (x_max, x_min) = self.get_max_min_axis(0);
        x_max - x_min + 1
    }
}

impl fmt::Display for ImgVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let transformed = self.to_char_vec();
        let w = self.get_width();

        write!(f, "\n")?;
        for (i, ch) in transformed.iter().enumerate() {
            if i > 0 && i % w as usize == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", ch)?;
        }

        Ok(())
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Color {
    BLACK,
    WHITE,
}

enum TurnDirection {
    LEFT,
    RIGHT,
}

#[derive(Copy, Clone)]
enum FaceDirection {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position { x: x, y: y }
    }
}

impl From<(i64, i64)> for FaceDirection {
    fn from(val: (i64, i64)) -> FaceDirection {
        match val {
            (0, 1) => FaceDirection::UP,
            (1, 0) => FaceDirection::RIGHT,
            (0, -1) => FaceDirection::DOWN,
            (-1, 0) => FaceDirection::LEFT,
            _ => panic!("Unknown face direction: {:?}", val),
        }
    }
}

impl From<FaceDirection> for (i64, i64) {
    fn from(fd: FaceDirection) -> (i64, i64) {
        match fd {
            FaceDirection::UP => (0, 1),
            FaceDirection::LEFT => (-1, 0),
            FaceDirection::DOWN => (0, -1),
            FaceDirection::RIGHT => (1, 0),
        }
    }
}

impl From<i64> for Color {
    fn from(val: i64) -> Self {
        match val {
            0 => Color::BLACK,
            1 => Color::WHITE,
            _ => panic!("Invalid color representation: {}", val),
        }
    }
}

impl From<Color> for i64 {
    fn from(color: Color) -> i64 {
        match color {
            Color::WHITE => 1,
            Color::BLACK => 0,
        }
    }
}

impl From<i64> for TurnDirection {
    fn from(val: i64) -> Self {
        match val {
            0 => TurnDirection::LEFT,
            1 => TurnDirection::RIGHT,
            _ => panic!("Invalid turn direction representation: {}", val),
        }
    }
}

impl From<TurnDirection> for i64 {
    fn from(dir: TurnDirection) -> i64 {
        match dir {
            TurnDirection::LEFT => 0,
            TurnDirection::RIGHT => 1,
        }
    }
}

impl FaceDirection {}

struct Robot {
    position: Position,
    face_direction: FaceDirection,
    cpu: Cpu,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            position: Position::new(0, 0),
            face_direction: FaceDirection::UP,
            cpu: Cpu::new(input::INPUT),
        }
    }

    fn update_face_direction(&mut self, turn_dir: TurnDirection) {
        self.face_direction = match (self.face_direction, turn_dir) {
            (FaceDirection::UP, TurnDirection::LEFT) => FaceDirection::LEFT,
            (FaceDirection::UP, TurnDirection::RIGHT) => FaceDirection::RIGHT,
            (FaceDirection::RIGHT, TurnDirection::LEFT) => FaceDirection::UP,
            (FaceDirection::RIGHT, TurnDirection::RIGHT) => FaceDirection::DOWN,
            (FaceDirection::DOWN, TurnDirection::LEFT) => FaceDirection::RIGHT,
            (FaceDirection::DOWN, TurnDirection::RIGHT) => FaceDirection::LEFT,
            (FaceDirection::LEFT, TurnDirection::LEFT) => FaceDirection::DOWN,
            (FaceDirection::LEFT, TurnDirection::RIGHT) => FaceDirection::UP,
        };
    }

    fn update_position(&mut self) {
        let (dx, dy) = match self.face_direction {
            FaceDirection::UP => (0, 1),
            FaceDirection::RIGHT => (1, 0),
            FaceDirection::DOWN => (0, -1),
            FaceDirection::LEFT => (-1, 0),
        };
        self.position.x += dx;
        self.position.y += dy;
    }

    fn paint(&mut self, start_panel: Color) -> HashMap<Position, Color> {
        let mut painted: HashMap<Position, Color> = HashMap::new();
        let mut is_color_out: bool = true;
        self.cpu.push_input(start_panel.into());

        while let Some(output) = self.cpu.run_until_output() {
            match is_color_out {
                true => {
                    let color: Color = output.into();
                    painted.insert(self.position, color);
                }
                false => {
                    self.update_face_direction(output.into());
                    self.update_position();
                    let color: Color =
                        (*painted.get(&self.position).unwrap_or(&Color::BLACK)).into();
                    self.cpu.push_input(color.into())
                }
            }
            is_color_out = !is_color_out;
        }

        painted
    }
}
