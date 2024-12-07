use std::collections::HashMap;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn in_rect(self, top_left: Coord, bottom_right: Coord) -> bool {
        top_left.x <= self.x && self.x <= bottom_right.x && top_left.y <= self.y && self.y <= bottom_right.y 
    }
}

impl ops::Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn parse_with_coords<T, F>(input: &str, parse_fn: F) -> (Coord, HashMap<Coord, T>)
where
    F: Fn(&char) -> Option<T>,
{
    let mut map = HashMap::new();
    let mut y_max: i32 = 0;
    let mut x_max: i32 = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        y_max = y_max.max(y as i32);
        line.chars().enumerate().for_each(|(x, ch)| {
            x_max = x_max.max(x as i32);
            if let Some(val) = parse_fn(&ch) {
                map.insert(
                    Coord {
                        x: x as i32,
                        y: y as i32,
                    },
                    val,
                );
            }
        });
    });

    (Coord { x: x_max, y: y_max }, map)
}
