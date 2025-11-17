use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(3068, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(1514285714288, r);
    }
}

const SHAPES: [&str; 5] = [
    r"####",
    r".#.
###
.#.",
    r"..#
..#
###",
    r"#
#
#
#",
    r"##
##",
];
const DRAWING_ENABLED: usize = 0;

type Chamber = Vec<Vec<char>>;
type Shape = Vec<Vec<char>>;
// type Vec2i = (i32, i32);

fn read_shape_at(shape: &Shape, x: i32, y: i32) -> bool {
    if x < 0 || x >= shape[0].len() as i32 || y < 0 || y >= shape.len() as i32 {
        panic!("Cannot read outside shape");
    }
    let shape_height = shape.len();
    shape[shape_height - 1 - (y as usize)][x as usize] != '.'
}

fn shape_height(shape: &Shape) -> usize {
    shape.len()
}

fn shape_width(shape: &Shape) -> usize {
    shape[0].len()
}

#[allow(dead_code)]
fn chamber_height(chamber: &Chamber) -> usize {
    chamber.len()
}

fn chamber_width(chamber: &Chamber) -> usize {
    chamber[0].len()
}

fn read_chamber_at(chamber: &Chamber, x: i32, y: i32) -> bool {
    if x < 0 || x >= chamber_width(chamber) as i32 || y < 0 {
        return true;
    }
    if y >= chamber_height(chamber) as i32 {
        return false;
    }
    chamber[y as usize][x as usize] != '.'
}

fn write_chamber_at(chamber: &mut Chamber, x: i32, y: i32, value: char) {
    if x < 0 || x >= chamber_width(chamber) as i32 || y < 0 {
        panic!("Cannot write outside chamber");
    }
    while y >= chamber_height(chamber) as i32 {
        chamber.push(vec!['.'; chamber_width(chamber)]);
    }
    chamber[y as usize][x as usize] = value;
}

#[allow(dead_code)]
fn print_chamber(chamber: &Chamber) {
    for y in 0..chamber_height(chamber) {
        print!("|");
        for c in chamber[chamber.len() - 1 - y].iter().take(chamber_width(chamber)) {
            print!("{}", c);
        }
        println!("|");
    }
    println!();
}

fn print_chamber_with_falling_rock(
    chamber: &Chamber,
    rock_shape: &Shape,
    rock_x: i32,
    rock_y: i32,
) {
    let mut chamber = chamber.clone();
    draw_rock(rock_shape, rock_x, rock_y, &mut chamber, '@');
    print_chamber(&chamber);
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn collision_test(chamber: &Chamber, shape: &Shape, x: i32, y: i32) -> bool {
    if x < 0 || (x + shape_width(shape) as i32) > chamber_width(chamber) as i32 {
        return true;
    }
    for sy in 0..shape_height(shape) as i32 {
        for sx in 0..shape_width(shape) as i32 {
            if read_shape_at(shape, sx, sy) && read_chamber_at(chamber, x + sx, y + sy) {
                return true;
            }
        }
    }
    false
}

fn draw_rock(rock_shape: &Shape, x: i32, y: i32, chamber: &mut Chamber, symbol: char) {
    for sy in 0..shape_height(rock_shape) as i32 {
        for sx in 0..shape_width(rock_shape) as i32 {
            if read_shape_at(rock_shape, sx, sy) {
                write_chamber_at(chamber, x + sx, y + sy, symbol);
            }
        }
    }
}

fn drop_rock(
    x: i32,
    y: i32,
    rock_shape: &Shape,
    chamber: &mut Chamber,
    jet_index: &mut usize,
    jets: &[char],
    pruning_enabled: bool,
) -> usize {
    let mut x = x;
    let mut y = y;
    if DRAWING_ENABLED >= 1 {
        println!("New rock");
        print_chamber_with_falling_rock(chamber, rock_shape, x, y);
    }

    loop {
        let c = jets[*jet_index];
        assert!(c == '<' || c == '>');
        let try_x = x + if jets[*jet_index] == '<' { -1 } else { 1 };
        let is_collision = collision_test(chamber, rock_shape, try_x, y);
        x = if is_collision { x } else { try_x };
        *jet_index += 1;
        *jet_index %= jets.len();

        if DRAWING_ENABLED == 2 {
            println!("Jet of gas:");
            print_chamber_with_falling_rock(chamber, rock_shape, x, y);
            println!("Fall 1 unit:");
        }

        let try_y = y - 1;
        let is_collision = collision_test(chamber, rock_shape, x, try_y);
        if is_collision {
            if DRAWING_ENABLED == 2 {
                print_chamber_with_falling_rock(chamber, rock_shape, x, y);
            }
            break;
        }
        y = try_y;
        if DRAWING_ENABLED == 2 {
            print_chamber_with_falling_rock(chamber, rock_shape, x, y);
        }
    }

    draw_rock(rock_shape, x, y, chamber, '#');

    // y += shape_height(rock_shape) as i32;
    if pruning_enabled {
        prune_chamber(chamber, y, shape_height(rock_shape))
    } else {
        0
    }
}

fn solve(input_file: &str) -> usize {
    let jets = input_file.trim().chars().collect::<Vec<_>>();
    let mut jet_index = 0;
    let mut shape_index = 0;
    let mut chamber: Chamber = vec!["-------".chars().collect::<Vec<_>>()];
    let shapes: Vec<Shape> = SHAPES
        .iter()
        .map(|s| {
            s.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for rock_id in 0..2022 {
        let mut y = chamber.len() as i32;
        y += 3;
        let x = 2;

        let rock_shape = &shapes[shape_index];
        drop_rock(x, y, rock_shape, &mut chamber, &mut jet_index, &jets, false);

        if DRAWING_ENABLED == 2 {
            println!("Chamber after rock id has fallen {}:", &rock_id);
            print_chamber(&chamber);
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }

        shape_index += 1;
        shape_index %= shapes.len();
    }

    chamber.len() - 1
}

fn prune_chamber(chamber: &mut Chamber, rock_y: i32, rock_height: usize) -> usize {
    for y in rock_y..rock_y + rock_height as i32 {
        let mut all_set = true;
        for x in 0..chamber_width(chamber) as i32 {
            if !read_chamber_at(chamber, x, y) {
                all_set = false;
                break;
            }
        }
        if all_set {
            panic!("All set");
        }
    }
    // println!("No pruning found");

    0
}

fn solve2(input_file: &str) -> usize {
    let jets = input_file.trim().chars().collect::<Vec<_>>();
    let mut jet_index = 0;
    let mut shape_index = 0;
    let mut chamber: Chamber = vec!["-------".chars().collect::<Vec<_>>()];
    let shapes: Vec<Shape> = SHAPES
        .iter()
        .map(|s| {
            s.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let total_iterations = 1000000000000usize;
    let sample_iterations = 2000;
    let mut total_drops = 0;
    let drop_rock_iteration =
        |rock_id: usize, chamber: &mut Chamber, shape_index: &mut usize, jet_index: &mut usize| {
            let len = chamber.len() as i32;
            let mut y = len;
            y += 3;
            let x = 2;

            let rock_shape = &shapes[*shape_index];
            drop_rock(x, y, rock_shape, chamber, jet_index, &jets, false);

            if DRAWING_ENABLED == 2 {
                println!("Chamber after rock id has fallen {}:", &rock_id);
                print_chamber(chamber);
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }

            *shape_index += 1;
            *shape_index %= shapes.len();
        };

    loop {
        let height1 = chamber.len();
        drop_rock_iteration(total_drops, &mut chamber, &mut shape_index, &mut jet_index);
        total_drops += 1;
        if total_drops > sample_iterations && (chamber.len() == height1 + 1) {
            break;
        }
    }

    let height1 = chamber.len();
    let drop_count1 = total_drops;
    let shape_index1 = shape_index;
    const SAMPLE_HEIGHT: usize = 1000;
    let fingerprint = &chamber[chamber.len().saturating_sub(SAMPLE_HEIGHT)..].to_vec();
    // println!("fingerprint: {}", fingerprint.len());
    loop {
        drop_rock_iteration(total_drops, &mut chamber, &mut shape_index, &mut jet_index);
        total_drops += 1;
        let new_fingerprint = &chamber[chamber.len().saturating_sub(SAMPLE_HEIGHT)..].to_vec();
        if fingerprint == new_fingerprint && shape_index == shape_index1 {
            break;
        }
    }

    let height2 = chamber.len();
    let drop_count2 = total_drops;
    let height_diff = height2 - height1;
    // let shape_index2 = shape_index;
    let drop_count_diff = drop_count2 - drop_count1;
    // println!("height_diff: {}", height_diff);
    // println!("drop_count_diff: {}", drop_count_diff);
    // println!("shape_index_diff: {}", shape_index2 - shape_index1);
    // println!("current height: {}", chamber.len());

    let drop_count_left = total_iterations - total_drops;
    // println!("drop_count_left: {}", drop_count_left);

    let height_additon = height_diff * (drop_count_left / drop_count_diff);
    // println!("height_additon: {}", height_additon);
    let drop_count_rest = drop_count_left % drop_count_diff;
    for _ in 0..drop_count_rest {
        drop_rock_iteration(total_drops, &mut chamber, &mut shape_index, &mut jet_index);
        total_drops += 1;
    }

    let height_diff2 = chamber.len() - height2;
    // println!("height_diff2: {}", height_diff2);

    let final_height = height2 + height_additon + height_diff2;
    // println!("final_height: {}", final_height);

    final_height - 1
}

fn main() {
    let input_file = fs::read_to_string("input17.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);
    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}
