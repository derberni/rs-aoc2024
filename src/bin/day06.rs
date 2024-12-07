use anyhow::*;
use aoc2024::{parse_with_coords, Coord};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const DIRECTIONS: &[Coord] = &[
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 0 },
];

fn part1_body(map: &HashMap<Coord, MapItem>, map_size: Coord) -> HashSet<Coord> {
    let mut position = map
        .iter()
        .find(|(_, &item)| item == MapItem::Guard)
        .unwrap()
        .0
        .clone();

    let mut visited: HashSet<Coord> = HashSet::new();

    let mut direction = 0;

    while position.in_rect(Coord { x: 0, y: 0 }, map_size) {
        visited.insert(position);
        let next_position = position + DIRECTIONS[direction];
        match map.get(&next_position) {
            Some(MapItem::Obstruction) => direction = (direction + 1) % 4,
            _ => position = next_position,
        }
    }

    visited
}

fn part1(map: &HashMap<Coord, MapItem>, map_size: Coord) -> u32 {
    let visited = part1_body(map, map_size);

    visited.len() as u32
}

fn part2(map: &HashMap<Coord, MapItem>, map_size: Coord) -> u32 {
    let starting_position = map
        .iter()
        .find(|(_, &item)| item == MapItem::Guard)
        .unwrap()
        .0
        .clone();

    let mut obstruction_candidates = part1_body(map, map_size);
    obstruction_candidates.remove(&starting_position);

    obstruction_candidates
        .iter()
        .fold(0, |acc, &new_obstruction| {
            let mut map_with_obstruction = map.clone();
            map_with_obstruction.insert(new_obstruction, MapItem::Obstruction);
            let mut position = starting_position;

            let mut visited = HashSet::new();
            let mut direction = 0;

            while position.in_rect(Coord { x: 0, y: 0 }, map_size) {
                visited.insert((position, direction));
                let next_position = position + DIRECTIONS[direction];
                match (visited.contains(&(next_position, direction)), map_with_obstruction.get(&next_position)) {
                    (true, _) => return acc + 1,
                    (false, Some(MapItem::Obstruction)) => direction = (direction + 1) % 4,
                    (false, _) => position = next_position,
                }
            }

            acc
        })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MapItem {
    Guard,
    Obstruction,
}

impl MapItem {
    fn parse(c: &char) -> Option<MapItem> {
        match c {
            '#' => Some(MapItem::Obstruction),
            '^' => Some(MapItem::Guard),
            _ => None,
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/06.txt")?;

    let (map_size, map) = parse_with_coords(&input, MapItem::parse);

    let part1_result = part1(&map, map_size);
    println!("Part 1 result: {}", part1_result);

    let part2_result = part2(&map, map_size);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let (map_size, map) = parse_with_coords(&input, MapItem::parse);
    assert_eq!(part1(&map, map_size), 41);
}

#[test]
fn part2_example() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let (map_size, map) = parse_with_coords(&input, MapItem::parse);
    assert_eq!(part2(&map, map_size), 6);
}
