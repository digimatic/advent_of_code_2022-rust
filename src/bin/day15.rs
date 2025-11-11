use advent_of_code_2022_rust::parse_utils;
use std::{collections::HashSet, fs, ops::Add};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT, 10);
        assert_eq!(26, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT, 20);
        assert_eq!(56000011, r);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn parse(input_file: &str) -> Vec<(Point, Point)> {
    let xss = input_file
        .lines()
        .map(parse_utils::parse_signed_numbers)
        .collect::<Vec<_>>();
    let mut sensors = Vec::new();
    for xs in xss.into_iter() {
        if let [x1, y1, x2, y2] = xs.as_slice() {
            sensors.push((Point::new(*x1, *y1), Point::new(*x2, *y2)));
        }
    }
    sensors
}

fn is_pos_clear(sensors: &Vec<(Point, Point)>, p: &Point) -> bool {
    for (s1, s2) in sensors {
        if p != s2 && manhattan_distance(p, s1) <= manhattan_distance(s1, s2) {
            return true;
        }
    }
    false
}

fn solve(input_file: &str, y: i64) -> usize {
    let sensors = parse(input_file);
    let max_distance = sensors
        .iter()
        .map(|(p1, p2)| manhattan_distance(p1, p2))
        .max()
        .unwrap();
    let all_points1 = sensors
        .iter()
        .cloned()
        .map(|(p1, _)| p1)
        .collect::<Vec<_>>();
    let all_points2 = sensors
        .iter()
        .cloned()
        .map(|(_, p2)| p2)
        .collect::<Vec<_>>();
    let all_points = all_points1
        .iter()
        .chain(all_points2.iter())
        .collect::<Vec<_>>();
    let min_pos_x = all_points.iter().map(|p| p.x).min().unwrap();
    let max_pos_x = all_points.iter().map(|p| p.x).max().unwrap();
    let mut line_cleared = HashSet::new();
    for x in min_pos_x - max_distance..=max_pos_x + max_distance {
        if is_pos_clear(&sensors, &Point::new(x, y)) {
            line_cleared.insert(x);
        }
    }
    line_cleared.len()
}

fn is_pos_clear2(sensors: &Vec<(Point, Point)>, p: &Point) -> Option<i64> {
    for (s1, s2) in sensors {
        let d = manhattan_distance(s1, s2) - manhattan_distance(p, s1);
        if d >= 0 {
            return Some(d);
        }
    }
    None
}

fn solve2(input_file: &str, r: i64) -> i64 {
    let sensors = parse(input_file);
    for y in 0..=r {
        let mut x = 0;
        loop {
            let d = is_pos_clear2(&sensors, &Point::new(x, y));
            if let Some(d) = d {
                x += d + 1;
                if x > r {
                    break;
                }
            } else {
                return 4000000 * x + y;
            }
        }
    }
    panic!("No solution found");
}

fn main() {
    let input_file = fs::read_to_string("input15.txt").unwrap();

    let r = solve(&input_file, 2000000);
    println!("{}", r);

    let r = solve2(&input_file, 4000000);
    println!("{}", r);
}
