use anyhow::*;
use aoc2024::{parse_with_coords, Coord};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

const DIRECTIONS: &[Coord] = &[
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 0 },
];

fn part1(input: &str) -> u64 {
    let (_map_size, trail_map) = parse_with_coords(input, |&c| c.to_digit(10));

    let top_coords = trail_map
        .iter()
        .filter_map(|(coord, c)| match c {
            9 => Some(*coord),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut accessible: HashMap<Coord, u64> = HashMap::new();

    top_coords.iter().for_each(|&top| {
        let mut visited = HashSet::new();
        visited.insert(top);

        let mut candidates = VecDeque::new();
        DIRECTIONS.iter().for_each(|dir| {
            let next_position = top + *dir;
            if let Some(&next_height) = trail_map.get(&next_position) {
                if 9u32.checked_sub(next_height).unwrap_or(99) == 1 {
                    candidates.push_back((next_position, next_height));
                };
            }
        });
        while candidates.len() > 0 {
            let (position, height) = candidates.pop_front().unwrap();
            if visited.contains(&position) {
                continue;
            }
            accessible
                .entry(position)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            visited.insert(position);
            DIRECTIONS.iter().for_each(|dir| {
                let next_position = position + *dir;
                if let Some(&next_height) = trail_map.get(&next_position) {
                    if height.checked_sub(next_height).unwrap_or(99) == 1 {
                        candidates.push_back((next_position, next_height));
                    };
                }
            });
        }
    });

    let trailheads = trail_map
        .iter()
        .filter_map(|(coord, c)| match c {
            0 => Some(*coord),
            _ => None,
        })
        .collect::<Vec<_>>();

    trailheads
        .iter()
        .fold(0, |acc, coord| match accessible.get(coord) {
            Some(n) => acc + n,
            None => acc,
        })
}

fn part2(input: &str) -> u64 {
    let (_map_size, trail_map) = parse_with_coords(input, |&c| c.to_digit(10));

    let top_coords = trail_map
        .iter()
        .filter_map(|(coord, c)| match c {
            9 => Some(*coord),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut accessible: HashMap<Coord, u64> = HashMap::new();

    top_coords.iter().for_each(|&top| {
        let mut candidates = VecDeque::new();
        DIRECTIONS.iter().for_each(|dir| {
            let next_position = top + *dir;
            if let Some(&next_height) = trail_map.get(&next_position) {
                if 9u32.checked_sub(next_height).unwrap_or(99) == 1 {
                    candidates.push_back((next_position, next_height));
                };
            }
        });
        while candidates.len() > 0 {
            let (position, height) = candidates.pop_front().unwrap();
            accessible
                .entry(position)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            DIRECTIONS.iter().for_each(|dir| {
                let next_position = position + *dir;
                if let Some(&next_height) = trail_map.get(&next_position) {
                    if height.checked_sub(next_height).unwrap_or(99) == 1 {
                        candidates.push_back((next_position, next_height));
                    };
                }
            });
        }
    });

    let trailheads = trail_map
        .iter()
        .filter_map(|(coord, c)| match c {
            0 => Some(*coord),
            _ => None,
        })
        .collect::<Vec<_>>();

    trailheads
        .iter()
        .fold(0, |acc, coord| match accessible.get(coord) {
            Some(n) => acc + n,
            None => acc,
        })
}

fn main() -> Result<()> {
    let input = read_to_string("input/10.txt")?;

    let part1_result = part1(&input);
    println!("Part 1 result: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    assert_eq!(part1(&input), 36);
}

#[test]
fn part2_example() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    assert_eq!(part2(&input), 81);
}
