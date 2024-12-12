use anyhow::*;
use aoc2024::{parse_with_coords, Coord};
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

const DIRECTIONS: &[Coord] = &[
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 0 },
];

fn part1(input: &str) -> u64 {
    let (_, garden_map) = parse_with_coords(input, |c| Some(*c));

    let mut counted = HashSet::new();

    let coords = garden_map.keys().cloned().collect::<Vec<_>>();

    coords.iter().fold(0, |acc, &c| {
        if counted.contains(&c) {
            return acc;
        };

        let label = garden_map[&c];

        let mut area = 0;
        let mut perimeter = 0;
        let mut patch_candidates = VecDeque::new();
        patch_candidates.push_back(c);

        while patch_candidates.len() > 0 {
            let candidate = patch_candidates.pop_front().unwrap();
            area += 1;
            counted.insert(candidate);
            let mut candidate_perimeter = 4;
            DIRECTIONS.iter().for_each(|dir| {
                let next = candidate + *dir;
                match garden_map.get(&next) {
                    Some(&next_label) => {
                        if next_label == label {
                            candidate_perimeter -= 1;
                            if !counted.contains(&next) && !patch_candidates.contains(&next) {
                                patch_candidates.push_back(next);
                            }
                        };
                    }
                    None => (),
                }
            });
            perimeter += candidate_perimeter;
        }

        acc + area * perimeter
    })
}

fn part2(input: &str) -> u64 {
    let (_, garden_map) = parse_with_coords(input, |c| Some(*c));

    let mut counted = HashSet::new();

    let coords = garden_map.keys().cloned().collect::<Vec<_>>();

    coords.iter().fold(0, |acc, &c| {
        if counted.contains(&c) {
            return acc;
        };

        let label = garden_map[&c];

        let mut area = 0;
        let mut left = HashSet::new();
        let mut right = HashSet::new();
        let mut top = HashSet::new();
        let mut bottom = HashSet::new();
        let mut patch_candidates = VecDeque::new();
        patch_candidates.push_back(c);

        while patch_candidates.len() > 0 {
            let candidate = patch_candidates.pop_front().unwrap();
            area += 1;
            counted.insert(candidate);
            match right.take(&Coord {
                x: candidate.x - 1,
                y: candidate.y,
            }) {
                Some(_) => (),
                None => {
                    left.insert(candidate);
                }
            };
            match left.take(&Coord {
                x: candidate.x + 1,
                y: candidate.y,
            }) {
                Some(_) => (),
                None => {
                    right.insert(candidate);
                }
            };
            match bottom.take(&Coord {
                x: candidate.x,
                y: candidate.y - 1,
            }) {
                Some(_) => (),
                None => {
                    top.insert(candidate);
                }
            };
            match top.take(&Coord {
                x: candidate.x,
                y: candidate.y + 1,
            }) {
                Some(_) => (),
                None => {
                    bottom.insert(candidate);
                }
            };
            DIRECTIONS.iter().for_each(|dir| {
                let next = candidate + *dir;
                match garden_map.get(&next) {
                    Some(&next_label) => {
                        if next_label == label
                            && !counted.contains(&next)
                            && !patch_candidates.contains(&next)
                        {
                            patch_candidates.push_back(next);
                        }
                    }
                    None => {}
                }
            });
        }

        let mut sides = 0;
        let _ = &[left, right].iter_mut().for_each(|set| {
            while set.len() > 0 {
                let first = set.iter().next().unwrap().clone();
                let mut i = 0;
                while let Some(_) = set.take(&Coord {
                    x: first.x,
                    y: first.y + i,
                }) {
                    i += 1;
                }
                let mut i = 1;
                while let Some(_) = set.take(&Coord {
                    x: first.x,
                    y: first.y - i,
                }) {
                    i += 1;
                }
                sides += 1;
            }
        });

        let _ = &[top, bottom].iter_mut().for_each(|set| {
            while set.len() > 0 {
                let first = set.iter().next().unwrap().clone();
                let mut i = 0;
                while let Some(_) = set.take(&Coord {
                    x: first.x + i,
                    y: first.y,
                }) {
                    i += 1;
                }
                let mut i = 1;
                while let Some(_) = set.take(&Coord {
                    x: first.x - i,
                    y: first.y,
                }) {
                    i += 1;
                }
                sides += 1;
            }
        });

        acc + area * sides
    })
}

fn main() -> Result<()> {
    let input = read_to_string("input/12.txt")?;

    let start = Instant::now();
    let part1_result = part1(&input);
    let end = Instant::now();
    println!("Part 1 result: {} ({:?})", part1_result, end - start);

    let start = Instant::now();
    let part2_result = part2(&input);
    let end = Instant::now();
    println!("Part 2 result: {} ({:?})", part2_result, end - start);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    assert_eq!(part1(&input), 1930);
}

#[test]
fn part2_example() {
    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    assert_eq!(part2(&input), 236);

    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    assert_eq!(part2(&input), 368);

    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    assert_eq!(part2(&input), 1206);
}
