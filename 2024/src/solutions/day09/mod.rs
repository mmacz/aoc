use crate::solver::Solver;

pub struct Problem;

#[derive(Clone, Copy, Debug)]
struct Block {
    is_file: bool,
    id: isize,
}

#[derive(Clone, Copy, Debug)]
struct Blob {
    pos: usize,
    size: usize,
}

type Filesystem = Vec<Block>;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        let mut fs: Filesystem = build_filesystem(input);
        rearrange_blocks(&mut fs);
        checksum(&fs)
    }

    fn solution2(&self, input: &str) -> Self::Ans2 {
        let mut fs: Filesystem = build_filesystem(input);
        rearrange_whole_blocks(&mut fs);
        checksum(&fs)
    }
}

fn build_filesystem(input: &str) -> Filesystem {
    let mut is_file: bool = true;
    let mut curr_id: isize = 0;
    input
        .chars()
        .flat_map(|chr| {
            let v: Filesystem = chr
                .to_digit(10)
                .map(|n| {
                    vec![
                        Block {
                            is_file,
                            id: if is_file { curr_id } else { -1 },
                        };
                        n as usize
                    ]
                })
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

    while left_idx < right_idx {
        let l: Block = disk[left_idx];
        let r: Block = disk[right_idx];
        match l.is_file {
            true => left_idx += 1,
            false => match r.is_file {
                false => right_idx -= 1,
                true => {
                    disk[left_idx] = disk[right_idx];
                    disk[right_idx] = l;
                }
            },
        }
    }
}

fn rearrange_whole_blocks(disk: &mut Filesystem) {
    let mut gaps: Vec<Blob> = disk
        .iter()
        .enumerate()
        .filter(|(_, block)| !block.is_file)
        .fold(Vec::<Blob>::new(), |mut acc, (pos, _)| {
            if let Some(last) = acc.last_mut() {
                if last.pos + last.size == pos {
                    last.size += 1;
                } else {
                    acc.push(Blob { pos, size: 1 });
                }
            } else {
                acc.push(Blob { pos, size: 1 });
            }
            acc
        });

    let mut files: Vec<Blob> = disk
        .iter()
        .enumerate()
        .filter(|(_, block)| block.is_file)
        .fold(Vec::<Blob>::new(), |mut acc, (pos, block)| {
            if let Some(last) = acc.last_mut() {
                if last.pos + last.size == pos && disk[last.pos].id == block.id {
                    last.size += 1;
                } else {
                    acc.push(Blob { pos, size: 1 });
                }
            } else {
                acc.push(Blob { pos, size: 1 });
            }
            acc
        });
    files.reverse();

    for file in files.iter_mut() {
        if let Some((gap_idx, gap)) = gaps
            .iter_mut()
            .enumerate()
            .find(|(_, gap)| gap.size >= file.size && gap.pos < file.pos)
        {
            for i in 0..file.size {
                disk[gap.pos + i] = disk[file.pos + i];
                disk[file.pos + i] = Block {
                    is_file: false,
                    id: -1,
                };
            }
            file.pos = gap.pos - file.size;
            gap.pos += file.size;
            gap.size -= file.size;
            if gap.size == 0 {
                gaps.remove(gap_idx);
            }
        }
    }
}

fn checksum(disk: &Filesystem) -> usize {
    disk.iter()
        .enumerate()
        .map(|(pos, block)| {
            if block.is_file {
                pos * block.id as usize
            } else {
                0
            }
        })
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

    #[test]
    fn test_day_09_filesystem_checksum_3() {
        let mut fs: Filesystem = build_filesystem(TEST_INPUT_2);
        rearrange_whole_blocks(&mut fs);
        assert_eq!(2858, checksum(&fs));
    }

    const TEST_INPUT_1: &str = "12345";
    const TEST_INPUT_2: &str = "2333133121414131402";
}
