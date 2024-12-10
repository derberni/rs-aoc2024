use anyhow::*;
use std::collections::{VecDeque};
use std::fs::read_to_string;

#[derive(Debug)]
enum DiskEntry {
    Free { len: u32 },
    File { len: u32, id: usize },
}

fn part1(input: &str) -> u64 {
    let mut entries = input
        .chars()
        .enumerate()
        .map(|(idx, c)| match idx % 2 {
            0 => DiskEntry::File {
                len: c.to_digit(10).unwrap(),
                id: idx / 2,
            },
            _ => DiskEntry::Free {
                len: c.to_digit(10).unwrap(),
            },
        })
        .collect::<VecDeque<_>>();

    let mut disk_idx = 0u64;
    let mut checksum = 0;

    while entries.len() > 0 {
        let front = entries.pop_front();

        match front {
            Some(DiskEntry::File { mut len, id }) => {
                while len > 0 {
                    checksum += disk_idx * id as u64;
                    disk_idx += 1;
                    len -= 1;
                }
            }
            Some(DiskEntry::Free { len: mut len_free }) => {
                while len_free > 0 {
                    match entries.pop_back() {
                        Some(DiskEntry::Free { len: _len }) => {}
                        Some(DiskEntry::File {
                            len: mut len_file,
                            id,
                        }) => {
                            while len_file > 0 && len_free > 0 {
                                checksum += disk_idx * id as u64;
                                disk_idx += 1;
                                len_file -= 1;
                                len_free -= 1;
                            }
                            if len_file > 0 {
                                entries.push_back(DiskEntry::File { len: len_file, id });
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
            None => break,
        }
    }

    checksum
}

fn part2(input: &str) -> u64 {
    let mut entries = input
        .chars()
        .enumerate()
        .map(|(idx, c)| match idx % 2 {
            0 => DiskEntry::File {
                len: c.to_digit(10).unwrap(),
                id: idx / 2,
            },
            _ => DiskEntry::Free {
                len: c.to_digit(10).unwrap(),
            },
        })
        .collect::<VecDeque<_>>();

    let mut tail = VecDeque::new();

    while entries.len() > 0 {
        match entries.pop_back() {
            Some(DiskEntry::Free { len }) => {
                tail.push_front(DiskEntry::Free { len });
            }
            Some(DiskEntry::File { id, len: len_file }) => {
                match entries.iter().position(|e| match e {
                    DiskEntry::Free { len: len_free } => *len_free >= len_file,
                    _ => false,
                }) {
                    Some(free_idx) => {
                        match entries.get_mut(free_idx) {
                            Some(DiskEntry::Free { len: len_free }) => {
                                *len_free -= len_file;
                            }
                            _ => {}
                        }
                        entries.insert(free_idx, DiskEntry::File { id, len: len_file });
                        tail.push_front(DiskEntry::Free { len: len_file });
                    }
                    _ => tail.push_front(DiskEntry::File { id, len: len_file }),
                }
            }
            None => {
                break;
            }
        }
    }

    tail.iter()
        .fold((0u64, 0u64), |(disk_index, checksum), entry| match entry {
            &DiskEntry::Free { len } => (disk_index + len as u64, checksum),
            &DiskEntry::File { len, id } => {
                let add_checksum: u64 = (disk_index..disk_index + len as u64)
                    .map(|i| i * id as u64)
                    .sum();
                (disk_index + len as u64, checksum + add_checksum)
            }
        })
        .1
}

fn main() -> Result<()> {
    let input = read_to_string("input/09.txt")?;

    let part1_result = part1(&input);
    println!("Part 1 result: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "2333133121414131402";

    assert_eq!(part1(&input), 1928);
}

#[test]
fn part2_example() {
    let input = "2333133121414131402";

    assert_eq!(part2(&input), 2858);
}
