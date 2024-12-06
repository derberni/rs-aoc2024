use anyhow::*;
use std::collections::HashMap;
use std::fs::read_to_string;

fn part1(coord_map: &HashMap<(i32, i32), char>) -> u32 {
    let neighbors = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    coord_map.iter().fold(0, |acc, ((x, y), c)| match c {
        &'X' => {
            acc + neighbors
                .clone()
                .into_iter()
                .fold(0, |n_acc, (x_dir, y_dir)| {
                    match coord_map.get(&(x + x_dir, y + y_dir)) {
                        Some('M') => match coord_map.get(&(x + 2 * x_dir, y + 2 * y_dir)) {
                            Some('A') => match coord_map.get(&(x + 3 * x_dir, y + 3 * y_dir)) {
                                Some('S') => n_acc + 1,
                                _ => n_acc,
                            },
                            _ => n_acc,
                        },
                        _ => n_acc,
                    }
                })
        }
        _ => acc,
    })
}

fn part2(coord_map: &HashMap<(i32, i32), char>) -> u32 {
    coord_map.iter().fold(0, |acc, ((x, y), c)| match c {
        &'A' => {
            let x_vec = vec![coord_map.get(&(x - 1, y - 1)), coord_map.get(&(x + 1, y + 1)), coord_map.get(&(x - 1, y + 1)), coord_map.get(&(x + 1, y - 1))];

            match x_vec.into_iter().collect::<Option<Vec<_>>>() {
                None => acc,
                Some(x_vec) => {
                    match x_vec[..] {
                        ['M', 'S', 'M', 'S'] => acc + 1,
                        ['M', 'S', 'S', 'M'] => acc + 1,
                        ['S', 'M', 'M', 'S'] => acc + 1,
                        ['S', 'M', 'S', 'M'] => acc + 1,
                        _ => acc,
                    }
                }
            }
        }
        _ => acc,
    })
}

fn parse_with_coords(input: &str) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            map.insert((x as i32, y as i32), ch);
        });
    });

    map
}

fn main() -> Result<()> {
    let input = read_to_string("input/04.txt")?;
    let coords = parse_with_coords(&input);

    let part1_result = part1(&coords);
    println!("Part 1 result: {}", part1_result);

    let part2_result = part2(&coords);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part1(&parse_with_coords(&input)), 18);
}

#[test]
fn part2_example() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part2(&parse_with_coords(&input)), 9);
}
