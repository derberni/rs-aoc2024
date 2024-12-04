use anyhow::*;
use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(&input).fold(0u32, |acc, c| {
        let (_, [left, right]) = c.extract();
        let (left, right) = (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap());

        acc + (left * right)
    })
}

fn part2(input: &str) -> u32 {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let captures = mul_re
        .captures_iter(input)
        .merge_by(do_re.captures_iter(input), |x, y| {
            match (x.get(0), y.get(0)) {
                (Some(x_match), Some(y_match)) => x_match.start() <= y_match.start(),
                _ => panic!("can not order captures")
            }
        });

    captures
        .fold((0u32, true), |(acc, enabled), c| match c.len() {
            2 => match c.extract() {
                (_, ["do()"]) => (acc, true),
                (_, ["don't()"]) => (acc, false),
                _ => panic!("don't"),
            },
            3 => {
                if enabled {
                    let (_, [left, right]) = c.extract();
                    let (left, right) = (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap());

                    return (acc + (left * right), true)
                }
                (acc, false)
            },
            _ => panic!()
        })
        .0
}
fn main() -> Result<()> {
    let input = read_to_string("input/03.txt")?;

    let part1_result = part1(&input);
    println!("Part 1 result: {}", part1_result);
    
    let part2_result = part2(&input);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(input), 161);
}

#[test]
fn part2_example() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(input), 48);
}