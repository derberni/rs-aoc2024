use anyhow::*;
use aoc2024::{parse_with_coords, Coord};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Obj {
    Wall,
    Robot,
    Crate,
    LCrate,
}

fn part1(input: &str) -> u64 {
    let (map, directions) = input.split("\n\n").take(2).collect_tuple().unwrap();
    let (_, mut warehouse) = parse_with_coords(map, |c| match c {
        '#' => Some(Obj::Wall),
        'O' => Some(Obj::Crate),
        '@' => Some(Obj::Robot),
        _ => None,
    });

    let mut robot = *warehouse.iter().find(|&(c, o)| o == &Obj::Robot).unwrap().0;
    warehouse.remove(&robot);

    directions.chars().for_each(|d| {
        let direction = match d {
            '<' => Coord { x: -1, y: 0 },
            '>' => Coord { x: 1, y: 0 },
            '^' => Coord { x: 0, y: -1 },
            'v' => Coord { x: 0, y: 1 },
            _ => return,
        };
        let next_robot = robot + direction;
        let mut next_pos = next_robot;
        let mut move_crates = Vec::new();
        let mut wall = false;
        loop {
            match warehouse.get(&next_pos) {
                Some(Obj::Wall) => {
                    wall = true;
                    break;
                }
                Some(Obj::Crate) => {
                    move_crates.push(next_pos);
                    next_pos = next_pos + direction;
                }
                _ => {
                    break;
                }
            }
        }
        if !wall {
            move_crates.iter().rev().for_each(|&pos| {
                let cr = warehouse.remove(&pos).unwrap();
                warehouse.insert(pos + direction, cr);
            });
            robot = next_robot;
        }
    });

    warehouse.iter().fold(0, |acc, (c, obj)| match obj {
        Obj::Crate => acc + c.x as u64 + 100 * c.y as u64,
        _ => acc,
    })
}

fn part2(input: &str) -> u64 {
    let (map, directions) = input.split("\n\n").take(2).collect_tuple().unwrap();
    let (map_size, mut warehouse) = parse_with_coords(map, |c| match c {
        '#' => Some(Obj::Wall),
        'O' => Some(Obj::Crate),
        '@' => Some(Obj::Robot),
        _ => None,
    });

    let mut sorted_warehouse = warehouse.iter().collect_vec();
    sorted_warehouse.sort_by(|(a, _), (b, _)| a.x.cmp(&b.x).reverse());

    let mut warehouse = HashMap::new();
    sorted_warehouse.iter().for_each(|(&c, &o)| match o {
        Obj::Wall => {
            warehouse.insert(Coord { x: c.x * 2, y: c.y }, Obj::Wall);
            warehouse.insert(
                Coord {
                    x: c.x * 2 + 1,
                    y: c.y,
                },
                Obj::Wall,
            );
        }
        Obj::Crate => {
            warehouse.insert(Coord { x: c.x * 2, y: c.y }, Obj::LCrate);
            warehouse.insert(
                Coord {
                    x: c.x * 2 + 1,
                    y: c.y,
                },
                Obj::Crate,
            );
        }
        Obj::Robot => {
            warehouse.insert(Coord { x: c.x * 2, y: c.y }, Obj::Robot);
        }
        _ => {}
    });

    let mut robot = *warehouse.iter().find(|&(c, o)| o == &Obj::Robot).unwrap().0;
    warehouse.remove(&robot);

    directions.chars().for_each(|d| {
        let direction = match d {
            '<' => Coord { x: -1, y: 0 },
            '>' => Coord { x: 1, y: 0 },
            '^' => Coord { x: 0, y: -1 },
            'v' => Coord { x: 0, y: 1 },
            _ => return,
        };
        let next_robot = robot + direction;
        let mut next_pos = vec![next_robot];
        let mut move_crates = VecDeque::new();
        let mut wall = false;
        while let Some(candidate) = next_pos.pop() {
            if move_crates.contains(&candidate) {
                continue;
            }
            match warehouse.get(&candidate) {
                Some(Obj::Wall) => {
                    wall = true;
                    break;
                }
                Some(Obj::Crate) => {
                    move_crates.push_front(candidate);
                    move_crates.push_front(candidate + Coord { x: -1, y: 0 });

                    next_pos.push(candidate + direction + Coord { x: -1, y: 0 });
                    next_pos.push(candidate + direction);
                }
                Some(Obj::LCrate) => {
                    move_crates.push_front(candidate);
                    move_crates.push_front(candidate + Coord { x: 1, y: 0 });

                    next_pos.push(candidate + direction + Coord { x: 1, y: 0 });
                    next_pos.push(candidate + direction);
                }
                _ => {}
            }
        }
        if !wall {
            while let Some(m) = move_crates.pop_front() {
                if warehouse.contains_key(&(m + direction)) {
                    move_crates.push_back(m);
                } else {
                    let cr = warehouse.remove(&m).unwrap();
                    warehouse.insert(m + direction, cr);
                }
            }
            robot = next_robot;
        }
    });

    let mut clone = warehouse.clone();
    clone.insert(robot, Obj::Robot);
    dbg!(Warehouse(
        Coord {
            x: map_size.x * 2 + 1,
            y: map_size.y
        },
        clone
    ));

    warehouse.iter().fold(0, |acc, (c, obj)| match obj {
        Obj::LCrate => acc + c.x as u64 + 100 * c.y as u64,
        _ => acc,
    })
}

fn main() -> Result<()> {
    let input = read_to_string("input/15.txt")?;

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

struct Warehouse(Coord, HashMap<Coord, Obj>);

impl Debug for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let map_size = self.0;
        let warehouse = &self.1;

        let mut res = String::with_capacity((map_size.x as usize + 1) * map_size.y as usize + 1);
        res.push('\n');

        (0..map_size.y + 1).for_each(|y| {
            (0..map_size.x + 1).for_each(|x| match warehouse.get(&Coord { x, y }) {
                Some(Obj::Wall) => res.push('#'),
                Some(Obj::Crate) => res.push(']'),
                Some(Obj::LCrate) => res.push('['),
                Some(Obj::Robot) => res.push('@'),
                _ => res.push('.'),
            });
            res.push('\n');
        });

        f.write_str(&res)
    }
}

#[test]
fn part1_small_example() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    assert_eq!(part1(&input), 2028);
}

#[test]
fn part1_example() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    assert_eq!(part1(&input), 10092);
}

#[test]
fn part2_small_example() {
    let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";
    assert_eq!(part2(&input), 9021);
}

#[test]
fn part2_example() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    assert_eq!(part2(&input), 9021);
}
