use anyhow::*;
use aoc2024::{parse_with_coords, Coord};
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn parse_antennas(input: &str) -> (Coord, HashMap<char, Vec<Coord>>) {
    let (map_size, antenna_map) =
        parse_with_coords(input, |c| if c == &'.' { None } else { Some(*c) });

    let mut antennas = HashMap::new();
    antenna_map
        .iter()
        .for_each(|(&coord, &id)| match antennas.entry(id) {
            Entry::Vacant(e) => {
                e.insert(vec![coord]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(coord);
            }
        });

    (map_size, antennas)
}

fn part1(input: &str) -> u64 {
    let (map_size, antennas) = parse_antennas(input);

    let mut resonances = HashSet::new();
    antennas.values().for_each(|coords| {
        coords.iter().permutations(2).for_each(|coord_pair| {
            let &a = coord_pair[0];
            let &b = coord_pair[1];
            let dist = a - b;
            if (a + dist).in_rect(Coord { x: 0, y: 0 }, map_size) {
                resonances.insert(a + dist);
            };
            if (b - dist).in_rect(Coord { x: 0, y: 0 }, map_size) {
                resonances.insert(b - dist);
            };
        })
    });

    resonances.len() as u64
}

fn part2(input: &str) -> u64 {
    let (map_size, antennas) = parse_antennas(input);

    let mut resonances = HashSet::new();
    antennas.values().for_each(|coords| {
        coords.iter().permutations(2).for_each(|coord_pair| {
            let &a = coord_pair[0];
            let &b = coord_pair[1];
            let dist = a - b;
            let mut i = 0;
            while (a + dist * i).in_rect(Coord { x: 0, y: 0 }, map_size) {
                resonances.insert(a + dist * i);
                i += 1;
            }
            let mut i = 0;
            if (b - dist * i).in_rect(Coord { x: 0, y: 0 }, map_size) {
                resonances.insert(b - dist * i);
                i += 1;
            };
        })
    });

    resonances.len() as u64
}

fn main() -> Result<()> {
    let input = read_to_string("input/08.txt")?;

    let part1_result = part1(&input);
    println!("Part 1 result: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    assert_eq!(part1(&input), 14);
}

#[test]
fn part2_example() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    assert_eq!(part2(&input), 34);
}
