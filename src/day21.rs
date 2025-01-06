use itertools::Itertools;
use std::cmp::min_by_key;
use std::collections::HashMap;
use std::iter::once;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Press {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Press {
    fn cost(&self) -> u64 {
        match self {
            Press::Left => 3,
            Press::Up | Press::Down => 2,
            Press::A | Press::Right => 1,
        }
    }

    fn position(&self) -> (i8, i8) {
        match self {
            Press::Up => (0, 1),
            Press::A => (0, 2),
            Press::Left => (1, 0),
            Press::Down => (1, 1),
            Press::Right => (1, 2),
        }
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day21, part1)]
pub fn part1(input: &[String]) -> u64 {
    part(input, 1)
}

#[aoc(day21, part2)]
pub fn part2(input: &[String]) -> u64 {
    part(input, 24)
}

fn part(input: &[String], depth: u8) -> u64 {
    let mut cache = HashMap::new();

    input
        .iter()
        .map(|line| {
            let num = line
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0);
            let numerical_path = numeric_keypad_path(line);
            let len = directional_keypad_path_complexity(numerical_path, 0, depth, &mut cache);
            num * len
        })
        .sum()
}

fn directional_keypad_path_complexity(
    path: Vec<Press>,
    depth: u8,
    max_depth: u8,
    cache: &mut HashMap<(Press, Press, u8), u64>,
) -> u64 {
    once(Press::A)
        .chain(path)
        .tuple_windows()
        .map(|(a, b)| min_path_step_cost(a, b, depth, max_depth, cache))
        .sum()
}

fn min_path_step_cost(
    a: Press,
    b: Press,
    depth: u8,
    max_depth: u8,
    cache: &mut HashMap<(Press, Press, u8), u64>,
) -> u64 {
    if let Some(&cost) = cache.get(&(a, b, depth)) {
        cost
    } else {
        let (curr_y, curr_x) = a.position();
        let (y, x) = b.position();
        let path = moves_between_positions(curr_y, curr_x, y, x, false);
        let result = if depth == max_depth {
            path.len() as u64
        } else {
            directional_keypad_path_complexity(path, depth + 1, max_depth, cache)
        };
        cache.insert((a, b, depth), result);
        result
    }
}

fn numeric_keypad_path(line: &str) -> Vec<Press> {
    let mut result = Vec::new();
    let (mut curr_y, mut curr_x) = (0i8, 2i8);

    for c in line.chars() {
        let (y, x) = match c {
            'A' => (0, 2),
            '0' => (0, 1),
            _ => {
                let n = c as i8 - '0' as i8;
                (((n as f64) / 3f64).ceil() as i8, (n - 1) % 3)
            }
        };
        result.extend(moves_between_positions(curr_y, curr_x, y, x, true));
        curr_y = y;
        curr_x = x;
    }
    result
}

fn moves_between_positions(
    curr_y: i8,
    curr_x: i8,
    y: i8,
    x: i8,
    positive_y_up: bool,
) -> Vec<Press> {
    let dy = y - curr_y;
    let dx = x - curr_x;
    let dy = if positive_y_up { -dy } else { dy };

    let horizontal =
        vec![if dx > 0 { Press::Right } else { Press::Left }; dx.unsigned_abs() as usize];
    let vertical = vec![if dy > 0 { Press::Down } else { Press::Up }; dy.unsigned_abs() as usize];

    let mut hv = horizontal
        .iter()
        .copied()
        .chain(vertical.iter().copied())
        .collect::<Vec<_>>();
    let mut vh = vertical
        .iter()
        .copied()
        .chain(horizontal.iter().copied())
        .collect::<Vec<_>>();

    hv.push(Press::A);
    vh.push(Press::A);

    if (curr_y, x) == (0, 0) {
        vh.clone()
    } else if (y, curr_x) == (0, 0) {
        hv.clone()
    } else {
        min_by_key(hv, vh, |path| cost_of_path(path))
    }
}

fn cost_of_path(path: &[Press]) -> u64 {
    path.iter().zip(0..).map(|(p, i)| p.cost() * i).sum()
}
