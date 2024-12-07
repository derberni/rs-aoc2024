
use anyhow::*;
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn variations<T>(n: usize, from: &[T]) -> Vec<Vec<&T>> {
    std::iter::repeat(from.iter())
        .take(n)
        .multi_cartesian_product()
        .collect::<Vec<Vec<&T>>>()
}

fn fold<F>(calibrations: Vec<(u64, Vec<u64>)>, operators: Vec<Operator>, match_fn: F) -> u64
where
    F: Fn(u64, u64, &Operator) -> u64,
{
    calibrations.iter().fold(0, |acc, (result, parts)| {
        if variations(parts.len() - 1, &operators).iter().any(|op| {
            let sum = parts[1..]
                .iter()
                .enumerate()
                .fold(parts[0], |acc, (i, part)| match_fn(acc, *part, op.get(i).unwrap()) );

            sum == *result
        }) {
            acc + result
        } else {
            acc
        }
    })
}

fn part1(input: &str) -> u64 {
    let calibrations = parse_calibrations(input);
    let operators = vec![Operator::Add, Operator::Multiply];

    fold(calibrations, operators, |acc, part, op| match op {
        Operator::Add => acc + part,
        Operator::Multiply => acc * part,
        Operator::Concat => panic!("concat operator should not occur for part1"),
    })
}

fn part2(input: &str) -> u64 {
    let calibrations = parse_calibrations(input);
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concat];

    fold(calibrations, operators, |acc, part, op| match op {
        Operator::Add => acc + part,
        Operator::Multiply => acc * part,
        Operator::Concat => acc * 10u64.pow(part.ilog10() + 1) + part,
    })
}

fn parse_calibrations(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (result, parts) = line.split_once(": ").unwrap();
            let parts = parts
                .split_whitespace()
                .map(|p| p.parse::<u64>().unwrap())
                .collect();

            (result.parse::<u64>().unwrap(), parts)
        })
        .collect()
}

fn main() -> Result<()> {
    let input = read_to_string("input/07.txt")?;

    let part1_result = part1(&input);
    println!("Part 1 result: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(part1(&input), 3749);
}

#[test]
fn part2_example() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(part2(&input), 11387);
}
