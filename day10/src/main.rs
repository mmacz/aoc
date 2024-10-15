use std::collections::HashSet;
use std::cmp::Ordering;

pub mod sanity_inputs;

fn asteroid_positions(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        // iterating over lines hence going down the map
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                // iterate over characters in lines hence going right to the map
                // move takes ownership
                .filter_map(move |(x, char)| match char {
                    '#' => Some((x as i32, y as i32)),
                    _ => None,
                })
        })
        .collect()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn direction(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let dx: i32 = b.0 - a.0;
    let dy: i32 = b.1 - a.1;
    let scale_factor: i32 = gcd(dx, dy);

    (dx / scale_factor, dy / scale_factor)
}

fn count_visible(curr_asteroid: (i32, i32), asteroids: &Vec<(i32, i32)>) -> usize {
    let mut directions: HashSet<(i32, i32)> = HashSet::new();
    for &a in asteroids.iter() {
        if curr_asteroid != a {
            directions.insert(direction(a, curr_asteroid));
        }
    }
    directions.len()
}

fn solution1(input: &str) -> ((i32, i32), usize) {
    let positions: Vec<(i32, i32)> = asteroid_positions(input);
    positions
        .iter()
        .map(|&a| {
            (a, count_visible(a, &positions))
        })
        .max_by(|&(_, cnt1), &(_, cnt2)| cnt1.cmp(&cnt2))
        .unwrap()
}

fn main() {
    println!("{:?}", solution1(sanity_inputs::INPUT1).0);
    //assert_eq!(8, solution1(sanity_inputs::INPUT1).1);
    //assert_eq!(33, solution1(sanity_inputs::INPUT2).1);
    //assert_eq!(210, solution1(sanity_inputs::INPUT3).1);
}
