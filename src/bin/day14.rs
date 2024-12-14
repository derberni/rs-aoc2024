use anyhow::*;
use aoc2024::{parse_coord, Coord};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::time::Instant;

struct Robot {
    position: Coord,
    velocity: Coord,
}

impl Robot {
    fn advance_in(&mut self, by: i32, map_size: &Coord) {
        self.position = (self.position + self.velocity * by + *map_size * by) % *map_size;
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (rem, _) = tag("p=")(input)?;
    let (rem, position) = parse_coord(rem)?;
    let (rem, _) = tag(" v=")(rem)?;
    let (rem, velocity) = parse_coord(rem)?;

    IResult::Ok((rem, Robot { position, velocity }))
}

fn part1(input: &str, map_size: Coord) -> u64 {
    let mut robots = separated_list1(newline, parse_robot)(input).unwrap().1;

    robots.iter_mut().for_each(|r| r.advance_in(100, &map_size));

    let mut robot_map: HashMap<Coord, u64> = HashMap::new();
    robots.iter().for_each(|r| {
        robot_map
            .entry(r.position)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    });

    let tl = robot_map.iter().filter(|(c, _)| {
        c.in_rect(
            Coord::zero(),
            Coord {
                x: map_size.x / 2 - 1,
                y: map_size.y / 2 - 1,
            },
        )
    });
    let tr = robot_map.iter().filter(|(c, _)| {
        c.in_rect(
            Coord {
                x: map_size.x / 2 + 1,
                y: 0,
            },
            Coord {
                x: map_size.x,
                y: map_size.y / 2 - 1,
            },
        )
    });
    let bl = robot_map.iter().filter(|(c, _)| {
        c.in_rect(
            Coord {
                x: 0,
                y: map_size.y / 2 + 1,
            },
            Coord {
                x: map_size.x / 2 - 1,
                y: map_size.y,
            },
        )
    });
    let br = robot_map.iter().filter(|(c, _)| {
        c.in_rect(
            Coord {
                x: map_size.x / 2 + 1,
                y: map_size.y / 2 + 1,
            },
            map_size,
        )
    });

    tl.fold(0, |acc, (_, n)| acc + n)
        * tr.fold(0, |acc, (_, n)| acc + n)
        * bl.fold(0, |acc, (_, n)| acc + n)
        * br.fold(0, |acc, (_, n)| acc + n)
}

fn part2(input: &str, map_size: Coord) -> u64 {
    let mut robots = separated_list1(newline, parse_robot)(input).unwrap().1;

    let mut i = 1;
    loop {
        let mut robot_map: RobotSet = RobotSet(map_size, HashSet::new());
        robots.iter_mut().for_each(|r| {
            r.advance_in(1, &map_size);
            robot_map.1.insert(r.position);
        });
        let (x_hist, y_hist) = robot_map.histograms();

        if x_hist.iter().max().unwrap_or(&0) > &20 && y_hist.iter().max().unwrap_or(&0) > &20 {
            dbg!(&robot_map);
            break;
        }
        i += 1;
    }

    i
}

struct RobotSet(Coord, HashSet<Coord>);

impl RobotSet {
    fn histograms(&self) -> (Vec<u32>, Vec<u32>) {
        let mut x_hist = vec![0; self.0.x as usize];
        let mut y_hist = vec![0; self.0.y as usize];

        self.1.iter().for_each(|Coord { x, y }| {
            x_hist[*x as usize] += 1;
            y_hist[*y as usize] += 1;
        });

        (x_hist, y_hist)
    }
}

impl Debug for RobotSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let map_size = self.0;
        let robots = &self.1;

        let mut res = String::with_capacity((map_size.x as usize + 1) * map_size.y as usize);

        (0..map_size.x).for_each(|x| {
            (0..map_size.y).for_each(|y| {
                if robots.contains(&Coord { x, y }) {
                    res.push('#');
                } else {
                    res.push('.');
                }
            });
            res.push('\n');
        });

        f.write_str(&res)
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/14.txt")?;

    let start = Instant::now();
    let part1_result = part1(&input, Coord { x: 101, y: 103 });
    let end = Instant::now();
    println!("Part 1 result: {} ({:?})", part1_result, end - start);

    let start = Instant::now();
    let part2_result = part2(&input, Coord { x: 101, y: 103 });
    let end = Instant::now();
    println!("Part 2 result: {} ({:?})", part2_result, end - start);

    Ok(())
}

#[test]
fn part1_example() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    let map_size = Coord { x: 11, y: 7 };
    assert_eq!(part1(&input, map_size), 12);
}
