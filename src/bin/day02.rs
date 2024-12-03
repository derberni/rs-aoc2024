use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_levels(line: String) -> Result<Vec<i32>> {
    Ok(line
        .split_whitespace()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn check_is_safe(levels: &Vec<i32>) -> bool {
    let diffs = levels
        .iter()
        .zip(levels.iter().skip(1))
        .map(|(l, s)| l - s)
        .collect::<Vec<_>>();

    let sign = diffs[0].signum();
    diffs.iter().all(|d| d.signum() == sign && d.abs().le(&3))
}

fn part1_line(line: String) -> Result<bool> {
    let levels = parse_levels(line)?;

    Ok(check_is_safe(&levels))
}

fn part2_line(line: String) -> Result<bool> {
    let levels = parse_levels(line)?;

    let mut combinations = vec![levels.clone()];
    (0..levels.len()).for_each(|i| {
        let mut new_diffs = levels.clone();
        new_diffs.remove(i);
        combinations.push(new_diffs);
    });

    Ok(combinations.iter().any(|d| check_is_safe(d)))
}

fn main() -> Result<()> {
    let input_file = BufReader::new(File::open("input/02.txt")?);
    let part1_result = input_file.lines().try_fold(0u32, |acc, line| {
        if part1_line(line?)? {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    });

    println!("Part 1: {}", part1_result?);

    let input_file = BufReader::new(File::open("input/02.txt")?);
    let part2_result = input_file.lines().try_fold(0u32, |acc, line| {
        if part2_line(line?)? {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    });

    println!("Part 2: {}", part2_result?);

    Ok(())
}

#[test]
fn part1_examples() {
    let tests = vec![
        ("7 6 4 2 1", true),
        ("1 2 7 8 9", false),
        ("9 7 6 2 1", false),
        ("1 3 2 4 5", false),
        ("8 6 4 4 1", false),
        ("1 3 6 7 9", true),
    ];

    tests.iter().for_each(|(input, expected)| {
        assert_eq!(
            part1_line(input.to_string()).unwrap(),
            *expected,
            "{input}`"
        )
    })
}

#[test]
fn part2_examples() {
    let tests = vec![
        ("7 6 4 2 1", true),
        ("1 2 7 8 9", false),
        ("9 7 6 2 1", false),
        ("1 3 2 4 5", true),
        ("8 6 4 4 1", true),
        ("1 3 6 7 9", true),
    ];

    tests.iter().for_each(|(input, expected)| {
        assert_eq!(
            part2_line(input.to_string()).unwrap(),
            *expected,
            "{input}`"
        )
    })
}
