use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(1651, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(1707, r);
    }
}

fn parse(input_file: &str) -> HashMap<String, (usize, Vec<String>)> {
    let rgx = Regex::new(r"Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow>[0-9]+); tunnels? leads? to valves? (?P<tunnels>[A-Z, ]{2,})").unwrap();
    let mut report = HashMap::new();
    for caps in rgx.captures_iter(input_file) {
        let name = caps["name"].to_string();
        let rate = caps["flow"].to_string();
        let tunnels = caps["tunnels"]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        report.insert(name, (rate.parse::<usize>().unwrap(), tunnels));
    }
    report
}

fn find_best_flow(report: &HashMap<String, (usize, Vec<String>)>) -> usize {
    let mut q = VecDeque::new();
    q.push_back(("AA".to_string(), 30, 0, HashSet::new()));
    let mut best_pressure = 0;
    let mut visited = HashSet::new();
    while !q.is_empty() {
        let (current, time_left, released_pressure, open_valves) = q.pop_front().unwrap();
        let mut sorted_valves: Vec<String> = open_valves.iter().cloned().collect();
        sorted_valves.sort();
        let key = (current.clone(), released_pressure, sorted_valves);
        if visited.contains(&(key)) {
            continue;
        }
        visited.insert(key);

        if released_pressure > best_pressure {
            best_pressure = released_pressure;
        }
        if time_left == 0 {
            continue;
        }
        if !open_valves.contains(&current) {
            let flow = report.get(&current).unwrap().0;
            if flow > 0 {
                let mut new_open_valves = open_valves.clone();
                new_open_valves.insert(current.clone());
                let new_time_left = time_left - 1;
                let new_released_pressure = released_pressure + flow * new_time_left;
                q.push_back((
                    current.clone(),
                    new_time_left,
                    new_released_pressure,
                    new_open_valves,
                ));
            }
        }

        for tunnel in report[&current].1.iter() {
            q.push_back((
                tunnel.clone(),
                time_left - 1,
                released_pressure,
                open_valves.clone(),
            ));
        }
    }
    best_pressure
}

fn solve(input_file: &str) -> usize {
    let report = parse(input_file);
    find_best_flow(&report)
}

fn find_best_flow2(report: &HashMap<String, (usize, Vec<String>)>) -> usize {
    let mut q = VecDeque::new();
    q.push_back(("AA".to_string(), "AA".to_string(), 26, 0, HashSet::new()));
    let mut best_pressure = 0;
    let mut visited = HashSet::new();
    let mut i = 0;
    while !q.is_empty() {
        i += 1;
        if i > 250000 {
            i = 0;
            println!("Queue length: {} ", q.len());
        }
        let (current, el_current, time_left, released_pressure, open_valves) =
            q.pop_front().unwrap();

        let key = (current.clone(), el_current.clone(), released_pressure);
        if visited.contains(&(key)) {
            continue;
        }

        visited.insert(key);

        if released_pressure > best_pressure {
            best_pressure = released_pressure;
            println!("best_pressure={}", best_pressure);
        }
        if time_left == 0 {
            continue;
        }
        let tunnels1 = &report[&current].1;
        let tunnels2 = &report[&el_current].1;
        for op1 in -1..tunnels1.len() as i32 {
            for op2 in -1..tunnels2.len() as i32 {
                if op1 == -1 && op2 == -1 && current == el_current {
                    continue;
                }
                let me_next = if op1 < 0 {
                    current.clone()
                } else {
                    tunnels1[op1 as usize].clone()
                };
                let el_next = if op2 < 0 {
                    el_current.clone()
                } else {
                    tunnels2[op2 as usize].clone()
                };

                let mut new_open_valves = open_valves.clone();
                let new_time_left = time_left - 1;
                let mut new_released_pressure = released_pressure;
                if op1 < 0 {
                    if !open_valves.contains(&current) {
                        let flow = report.get(&current).unwrap().0;
                        if flow > 0 {
                            new_open_valves.insert(current.clone());
                            new_released_pressure = released_pressure + flow * new_time_left;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                if op2 < 0 {
                    if !open_valves.contains(&el_current) {
                        let flow = report.get(&el_current).unwrap().0;
                        if flow > 0 {
                            new_open_valves.insert(el_current.clone());
                            new_released_pressure += flow * new_time_left;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                q.push_back((
                    me_next,
                    el_next,
                    new_time_left,
                    new_released_pressure,
                    new_open_valves,
                ));
            }
        }
    }
    best_pressure
}

fn solve2(input_file: &str) -> usize {
    let report = parse(input_file);
    find_best_flow2(&report)
}

fn main() {
    let input_file = fs::read_to_string("input16.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}
