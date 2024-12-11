use anyhow::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
enum Blink {
    Single(u64),
    Split(u64, u64),
}

fn blink(stone: u64) -> Blink {
    let n_digits = stone.checked_ilog10().unwrap_or(0) + 1;
    match (stone, n_digits % 2 == 0) {
        (0, _) => Blink::Single(1),
        (stone, true) => {
            let middle = 10u64.pow(n_digits / 2);
            Blink::Split(stone / middle, stone % middle)
        }
        _ => Blink::Single(stone * 2024),
    }
}

#[test]
fn test_blink() {
    assert_eq!(blink(0), Blink::Single(1));
    assert_eq!(blink(1), Blink::Single(2024));
    assert_eq!(blink(2024), Blink::Split(20, 24));
}

fn step(stone: u64, remaining_blinks: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if remaining_blinks == 0 {
        return 1;
    }
    if let Some(&n) = memo.get(&(stone, remaining_blinks)) {
        return n;
    }
    let res = match blink(stone) {
        Blink::Single(new_stone) => step(new_stone, remaining_blinks - 1, memo),
        Blink::Split(left, right) => {
            step(left, remaining_blinks - 1, memo) + step(right, remaining_blinks - 1, memo)
        }
    };
    memo.insert((stone, remaining_blinks), res);
    res
}

fn part1(input: &str) -> u64 {
    let stones = input.split_whitespace().map(|s| s.parse::<u64>().unwrap());

    let mut memo: HashMap<(u64, usize), u64> = HashMap::new();

    stones.fold(0, |acc, stone| acc + step(stone, 25, &mut memo))
}

fn part2(input: &str) -> u64 {
    let stones = input.split_whitespace().map(|s| s.parse::<u64>().unwrap());

    let mut memo: HashMap<(u64, usize), u64> = HashMap::new();

    stones.fold(0, |acc, stone| acc + step(stone, 75, &mut memo))
}

fn main() -> Result<()> {
    let input = read_to_string("input/11.txt")?;

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
    let input = "125 17";

    assert_eq!(part1(&input), 55312);
}

#[test]
fn part2_example() {
    let input = "125 17";

    assert_eq!(part2(&input), 81);
}
