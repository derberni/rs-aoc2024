use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn part1(distance_a: &Vec<u32>, distance_b: &Vec<u32>) -> Result<()> {
    let mut distance = 0u32;
    zip(distance_a, distance_b).for_each(|(a, b)| distance += a.abs_diff(*b));

    println!("part1: {}", distance);

    Ok(())
}

fn part2(distance_a: &Vec<u32>, distance_b: &Vec<u32>) -> Result<()> {
    let mut similarity = 0u32;
    
    distance_a.iter().for_each(|a| similarity += a * (distance_b.iter().filter(|b| a.eq(b)).count() as u32));

    println!("part2: {}", similarity);
    
    Ok(())
}

fn main() -> Result<()> {
    let input_file = BufReader::new(File::open("input/01.txt")?);

    let (mut distance_a, mut distance_b): (Vec<u32>, Vec<u32>) = input_file
        .lines()
        .map(|l| {
            l.unwrap()
                .split_once(' ')
                .map(|(a, b)| {
                    (
                        a.trim().parse::<u32>().unwrap(),
                        b.trim().parse::<u32>().unwrap(),
                    )
                })
                .unwrap()
        })
        .unzip();

    distance_a.sort();
    distance_b.sort();

    part1(&distance_a, &distance_b)?;
    part2(&distance_a, &distance_b)?;

    Ok(())
}
