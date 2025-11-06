use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14a() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let r = day14(input, false);
        assert_eq!(r, 24);
    }

    #[test]
    fn test_day14b() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let r = day14(input, true);
        assert_eq!(r, 93);
    }
}

fn day14(input: &str, part2: bool) -> usize {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let paths = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pointsstr| {
                    let captures = re.captures(pointsstr).unwrap();
                    let (_, [x, y]) = captures.extract();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut obstacles = HashSet::new();
    paths.iter().for_each(|points| {
        let mut iter = points.iter();
        let mut p = *iter.next().unwrap();
        for p2 in iter {
            obstacles.insert(p);
            while p != *p2 {
                if p.0 == p2.0 {
                    p.1 = if p2.1 > p.1 { p.1 + 1 } else { p.1 - 1 };
                } else {
                    p.0 = if p2.0 > p.0 { p.0 + 1 } else { p.0 - 1 };
                }
                obstacles.insert(p);
            }
        }
    });

    let max_y = obstacles.iter().map(|p| p.1).max().unwrap();

    let mut drop_sand = |p: (usize, usize)| -> bool {
        let mut p = p;
        while p.1 <= max_y {
            if !obstacles.contains(&(p.0, p.1 + 1)) {
                p = (p.0, p.1 + 1);
            } else if !obstacles.contains(&(p.0 - 1, p.1 + 1)) {
                p = (p.0 - 1, p.1 + 1);
            } else if !obstacles.contains(&(p.0 + 1, p.1 + 1)) {
                p = (p.0 + 1, p.1 + 1);
            } else {
                obstacles.insert(p);
                if part2 && p == (500, 0) {
                    return false;
                }
                return true;
            }
        }
        if part2 {
            obstacles.insert(p);
            true
        } else {
            false
        }
    };

    let mut count = 0;
    while drop_sand((500, 0)) {
        count += 1;
    }
    if part2 { count + 1 } else { count }
}

fn main() {
    let input = if let Some(filename) = std::env::args().nth(1) {
        fs::read_to_string(filename).expect("Failed to read input file")
    } else {
        fs::read_to_string("input14.txt").expect("Failed to read input file")
    };
    let r1 = day14(&input, false);
    println!("Day 14 part 1: {r1}");
    let r2 = day14(&input, true);
    println!("Day 14 part 2: {r2}");
}
