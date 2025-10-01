use crate::solver::Solver;

pub struct Problem;

type Block = (bool, usize); // file, id
type Filesystem = Vec<Block>;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        let mut fs: Filesystem = build_filesystem(input);
        rearrange_blocks(&mut fs);
        checksum(&fs)
    }

    fn solution2(&self, input: &str) -> Self::Ans1 {
        0
    }
}

fn build_filesystem(input: &str) -> Filesystem {
    let mut is_file: bool = true;
    let mut curr_id: usize = 0;
    input
        .chars()
        .flat_map(|chr| {
            let v: Filesystem = chr.to_digit(10)
                .map(|n| vec![(is_file, curr_id); n as usize])
                .unwrap_or_default();
            if is_file {
                curr_id += 1;
            }
            is_file = !is_file;
            v
        })
        .into_iter()
        .collect()
}

fn rearrange_blocks(disk: &mut Filesystem) -> () {
    let mut left_idx: usize = 0;
    let mut right_idx: usize = disk.len() - 1;

    while left_idx != right_idx {
        let l: Block = disk[left_idx];
        let r: Block = disk[right_idx];
        match l.0 {
            true => left_idx += 1,
            false => match r.0 {
                false => right_idx -= 1,
                true => {
                    disk[left_idx] = disk[right_idx];
                    disk[right_idx] = l;
                }
            },
        }
    }
}

fn checksum(disk: &Filesystem) -> usize {
    disk.iter()
        .enumerate()
        .filter(|(_, block)| block.0)
        .map(|(pos, block)| pos * block.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day09::*;

    #[test]
    fn test_day_09_filesystem_checksum_1() {
        let mut fs: Filesystem = build_filesystem(TEST_INPUT_1);
        rearrange_blocks(&mut fs);
        assert_eq!(60, checksum(&fs));
    }

    #[test]
    fn test_day_09_filesystem_checksum_2() {
        let mut fs: Filesystem = build_filesystem(TEST_INPUT_2);
        rearrange_blocks(&mut fs);
        assert_eq!(1928, checksum(&fs));
    }

    const TEST_INPUT_1: &str = "12345";
    const TEST_INPUT_2: &str = "2333133121414131402";
}
