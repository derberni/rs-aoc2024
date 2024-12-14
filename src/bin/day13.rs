use anyhow::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug)]
struct Machine {
    coefficients: [[i64; 2]; 2],
    constants: [i64; 2],
}

impl Machine {
    fn determinant(self: &Self) -> i64 {
        self.coefficients[0][0] * self.coefficients[1][1]
            - self.coefficients[1][0] * self.coefficients[0][1]
    }

    fn solve(self: &Self) -> Option<[i64; 2]> {
        match self.determinant() {
            0 => None,
            det => {
                let a = self.constants[0] * self.coefficients[1][1]
                    - self.constants[1] * self.coefficients[1][0];
                let b = self.coefficients[0][0] * self.constants[1]
                    - self.coefficients[0][1] * self.constants[0];
                if a % det != 0 || b % det != 0 {
                    None
                } else {
                    Some([a / det, b / det])
                }
            }
        }
    }
}

#[test]
fn test_solve_les() {
    assert_eq!(
        Machine {
            coefficients: [[94, 34], [22, 67]],
            constants: [8400, 5400]
        }
        .solve(),
        Some([80, 40])
    );
    assert_eq!(
        Machine {
            coefficients: [[26, 66], [67, 21]],
            constants: [12748, 12176]
        }
        .solve(),
        None
    );
    assert_eq!(
        Machine {
            coefficients: [[17, 86], [84, 37]],
            constants: [7870, 6450]
        }
        .solve(),
        Some([38, 86])
    );
    assert_eq!(
        Machine {
            coefficients: [[69, 23], [27, 71]],
            constants: [18641, 10279]
        }
        .solve(),
        None
    );
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (rem, button_a) = terminated(parse_button, newline)(input)?;
    let (rem, button_b) = terminated(parse_button, newline)(rem)?;
    let (rem, prize) = terminated(parse_prize, newline)(rem)?;

    IResult::Ok((
        rem,
        Machine {
            coefficients: [[button_a.0, button_a.1], [button_b.0, button_b.1]],
            constants: [prize.0, prize.1],
        },
    ))
}

fn parse_prize(input: &str) -> IResult<&str, (i64, i64)> {
    let (remaining, _) = tag("Prize: ")(input)?;
    separated_pair(parse_val, tag(", "), parse_val)(remaining)
}
fn parse_button(input: &str) -> IResult<&str, (i64, i64)> {
    let (remaining, _) = alt((tag("Button A: "), tag("Button B: ")))(input)?;
    separated_pair(parse_val, tag(", "), parse_val)(remaining)
}

fn parse_val(input: &str) -> IResult<&str, i64> {
    let (remaining, _) = alt((tag("X+"), tag("Y+"), tag("X="), tag("Y=")))(input)?;
    nom::character::complete::i64(remaining)
}

fn part1(input: &str) -> u64 {
    let machines = separated_list1(newline, parse_machine)(input).unwrap().1;

    machines
        .iter()
        .fold(0, |acc, machine| match machine.solve() {
            None => acc,
            Some([a, b]) => {
                if a < 100 && b < 100 {
                    acc + 3 * a as u64 + b as u64
                } else {
                    acc
                }
            }
        })
}

fn part2(input: &str) -> u64 {
    let machines = separated_list1(newline, parse_machine)(input).unwrap().1;

    machines
        .iter()
        .map(
            |&Machine {
                 coefficients,
                 constants,
             }| Machine {
                coefficients,
                constants: [constants[0] + 10000000000000, constants[1] + 10000000000000],
            },
        )
        .fold(0, |acc, machine| match machine.solve() {
            None => acc,
            Some([a, b]) => acc + 3 * a as u64 + b as u64,
        })
}

fn main() -> Result<()> {
    let input = read_to_string("input/13.txt")?;

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
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    assert_eq!(part1(&input), 480);
}

#[test]
fn part2_example() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    assert_eq!(part2(&input), 875318608908);
}
