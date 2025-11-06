use std::{cmp::Ordering, fs};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if *self == *other {
            return Some(Ordering::Equal);
        }
        if let Some(b) = compare_packets(self, other) {
            if b {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            None
        }
    }
}

fn parse_packet(s: &[char]) -> (Packet, &[char]) {
    if s.is_empty() {
        panic!("empty string")
    }
    let mut s = s;
    if let ['[', rest @ ..] = s {
        let mut v = Vec::new();
        s = rest;
        loop {
            if !s.is_empty() && s[0] != ']' {
                let (p, rest) = parse_packet(s);
                v.push(p);
                s = rest;
            }
            match s {
                rest @ [] => {
                    s = rest;
                    break;
                }
                [',', rest @ ..] => {
                    s = rest;
                }
                [']', rest @ ..] => {
                    s = rest;
                    break;
                }
                _ => panic!("Rest is not empty"),
            }
        }
        (Packet::List(v), s)
    } else {
        let mut consumed = 0;
        for (i, c) in s.iter().enumerate() {
            if c.is_numeric() {
                consumed = i + 1;
            } else {
                break;
            }
        }
        let number_chars: String = s[..consumed].iter().collect();
        let n = number_chars.parse::<u8>().unwrap();

        (Packet::Value(n), &s[consumed..])
    }
}

fn compare_packets(p1: &Packet, p2: &Packet) -> Option<bool> {
    match (p1, p2) {
        (Packet::Value(n1), Packet::Value(n2)) => {
            if n1 == n2 {
                None
            } else {
                Some(n1 < n2)
            }
        }
        (Packet::List(l1), Packet::List(l2)) => match (l1.as_slice(), l2.as_slice()) {
            (&[], &[_, ..]) => Some(true),
            ([_, ..], []) => Some(false),
            ([], []) => None,
            ([p1, l1new @ ..], [p2, l2new @ ..]) => {
                if let Some(r) = compare_packets(p1, p2) {
                    Some(r)
                } else {
                    compare_packets(&Packet::List(l1new.to_vec()), &Packet::List(l2new.to_vec()))
                }
            }
        },
        (&Packet::Value(v1), &Packet::List(_)) => {
            compare_packets(&Packet::List(vec![Packet::Value(v1)]), p2)
        }
        (&Packet::List(_), &Packet::Value(v2)) => {
            compare_packets(p1, &Packet::List(vec![Packet::Value(v2)]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    #[test]
    fn day13_part1_test1() {
        assert_eq!(13, part13(INPUT_EX1));
    }

    #[test]
    fn day13_part2_test1() {
        assert_eq!(140, part13_2(INPUT_EX1));
    }
}

fn part13(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let chunks = lines.chunks(3);
    let mut index_sum = 0;
    for (index, chunk) in chunks.enumerate() {
        let chars1: Vec<char> = chunk[0].chars().collect();
        let chars2: Vec<char> = chunk[1].chars().collect();
        let (packet1, rest1) = parse_packet(&chars1);
        let (packet2, rest2) = parse_packet(&chars2);
        if !rest1.is_empty() || !rest2.is_empty() {
            panic!("Expect no rest");
        }

        let r = compare_packets(&packet1, &packet2).expect("Cannot determine");
        if r {
            index_sum += index + 1;
        }
    }
    index_sum
}

fn part13_2(input: &str) -> usize {
    let mut packages = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (p, _) = parse_packet(l.chars().collect::<Vec<char>>().as_slice());
            p
        })
        .collect::<Vec<_>>();
    let divider_package_1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let divider_package_2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
    packages.push(divider_package_1.clone());
    packages.push(divider_package_2.clone());

    packages.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let i1 = 1 + packages
        .iter()
        .enumerate()
        .find(|(_, p)| **p == divider_package_1)
        .unwrap()
        .0;
    let i2 = 1 + packages
        .iter()
        .enumerate()
        .find(|(_, p)| **p == divider_package_2)
        .unwrap()
        .0;
    i1 * i2
}

fn main() {
    let input = fs::read_to_string("input13.txt").unwrap();
    let r1 = part13(&input);
    println!("Part 1: {r1}");
    let r2 = part13_2(&input);
    println!("Part 2: {r2}");
}
