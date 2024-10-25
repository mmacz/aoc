mod input;
use crate::solver::Solver;
use std::fmt;

const IMG_HEIGHT: usize = 6;
const IMG_WIDTH: usize = 25;

pub struct Problem;
pub struct ImgVec(Vec<u32>);

impl ImgVec {
       fn transform(&self) -> Vec<char> {
        self.0
            .iter()
            .map(|&ch| match ch {
                0 => ' ',
                1 => '#',
                _ => panic!("Invalid pattern")
            })
            .collect()
    }
}

impl fmt::Display for ImgVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let transformed = self.transform();
        write!(f, "\n")?;
        for (i, ch) in transformed.iter().enumerate() {
            if i > 0 && i % IMG_WIDTH == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", ch)?;
        }
        write!(f, "")
    }
}


fn get_layers(input: &str, width: usize, height: usize) -> Vec<Vec<u32>> {
    let sub_size = width * height;
    let chars: Vec<char> = input.chars().collect();
    chars
        .chunks_exact(sub_size)
        .map(|window| {
            window
                .iter()
                .map(|&c| c.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect()
}

fn count_digits(layer: &Vec<u32>, digit: u32) -> usize {
    layer
        .iter()
        .filter(|&&d| d == digit)
        .count()
}

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = ImgVec;

    fn solution1(&self) -> Self::Ans1 {
        let layers = get_layers(input::INPUT, IMG_WIDTH, IMG_HEIGHT);
        let zeros_cnt: Vec<usize> = layers
            .iter()
            .map(|l| count_digits(l, 0))
            .collect();
        let mzi: usize = zeros_cnt
            .iter()
            .enumerate()
            .min_by_key(|&(_, &num)| num)
            .map(|(index, _)| index)
            .unwrap() as usize;
        let ones_cnt = count_digits(&layers[mzi], 1);
        let twos_cnt = count_digits(&layers[mzi], 2);
        ones_cnt * twos_cnt
    }

    fn solution2(&self) -> Self::Ans2 {
        let layers = get_layers(input::INPUT, IMG_WIDTH, IMG_HEIGHT);
        let first_layer = layers[0].clone();

        ImgVec(first_layer
            .into_iter()
            .enumerate()
            .map(|(idx, pixel)| {
                layers
                    .iter()
                    .skip(1)
                    .fold(pixel, |acc, layer| {
                        let r: u32;
                        match acc {
                            0 => r = 0,
                            1 => r = 1,
                            2 => r = layer[idx],
                            _ => panic!("Invalid pixel value"),
                        }
                        r
                    })
                })
            .collect()
        )
    }
}

