use crate::{Solution, SolutionPair};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Block {
    Free,
    File(u64),
}

#[derive(Clone, Default)]
struct Disk(Vec<Block>);

impl std::fmt::Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|block| block.fmt(f).unwrap());
        Ok(())
    }
}

impl std::ops::Deref for Disk {
    type Target = Vec<Block>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Disk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free => f.write_str("."),
            Self::File(file_id) if *file_id < 10 => f.write_fmt(format_args!("{}", file_id)),
            Self::File(_) => f.write_str("X"),
        }
    }
}

impl Disk {
    fn checksum(&self) -> u64 {
        self.iter().enumerate().fold(0, |h, (pos, block)| {
            h + match block {
                Block::Free => 0,
                Block::File(file_id) => (pos as u64) * *file_id,
            }
        })
    }
}

fn prepare(input: &str) -> Disk {
    let mut disk = Disk::default();
    let mut file_id = 0;
    let mut is_free = false;
    input
        .trim_ascii_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .for_each(|num| {
            let block = if is_free {
                Block::Free
            } else {
                Block::File(file_id)
            };
            for _ in 0..num {
                disk.push(block);
            }
            if !is_free {
                file_id += 1;
            }
            is_free = !is_free;
        });
    disk
}

fn defragment(disk: &mut Disk) {
    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        if matches!(disk[left], Block::File(_)) {
            left += 1;
        } else if matches!(disk[right], Block::Free) {
            right -= 1;
        } else {
            disk[left] = disk[right];
            disk[right] = Block::Free;
            //eprintln!("{disk:?}");
        }
    }
}

fn solve_part1(input: &str) -> u64 {
    let mut disk = prepare(input);
    //eprintln!("{disk:?}");
    defragment(&mut disk);
    disk.checksum()
}

/// Find position of next free block.
fn find_next_free(disk: &Disk, mut from: usize) -> usize {
    while disk[from] != Block::Free {
        from += 1
    }
    from
}

/// Find position of next free span of length at least `min_len`.
fn find_next_free_span(disk: &Disk, start: usize, end: usize, min_len: usize) -> Option<usize> {
    let mut from = start;
    loop {
        from = find_next_free(disk, from);
        if from + min_len - 1 >= end {
            return None;
        }
        if let Some(non_free_pos) =
            (from..(from + min_len)).find(|pos| disk[*pos] != Block::Free)
        {
            from = non_free_pos;
        } else {
            return Some(from);
        }
    }
}

/// Move file blocks to free span
fn move_file(disk: &mut Disk, free_start: usize, file_start: usize) {
    let mut free = free_start;
    let mut file = file_start;
    if let Block::File(file_id) = disk[file] {
        while file < disk.len() && disk[file] == Block::File(file_id) {
            if disk[free] == Block::Free {
                disk[free] = disk[file];
                disk[file] = Block::Free;
                free += 1;
                file += 1;
            } else {
                panic!("not enough free blocks")
            }
        }
    } else {
        panic!("no file at start position")
    }
}

fn compact(disk: &mut Disk) {
    let mut leftmost_free = find_next_free(disk, 0);
    let mut right = disk.len() - 1;
    let mut next_file_id = *disk
        .iter()
        .filter_map(|block| match block {
            Block::Free => None,
            Block::File(file_id) => Some(file_id),
        })
        .max()
        .unwrap();
    while leftmost_free < right {
        if disk[right] == Block::File(next_file_id) {
            let mut file_len = 1;
            while right > 0 && disk[right - 1] == Block::File(next_file_id) {
                right -= 1;
                file_len += 1;
            }
            let file_start = right;
            if let Some(free_span) = find_next_free_span(disk, leftmost_free, file_start, file_len)
            {
                move_file(disk, free_span, file_start);
                leftmost_free = find_next_free(disk, leftmost_free);
            }
            if next_file_id == 0 {
                break;
            } else {
                next_file_id -= 1;
            }
        }
        right -= 1;
    }
}

fn solve_part2(input: &str) -> u64 {
    let mut disk = prepare(input);
    //eprintln!("{disk:?}");
    compact(&mut disk);
    disk.checksum()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 1928);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 2858);
    }
}
