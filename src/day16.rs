use std::collections::{HashSet, VecDeque};

type CostMap = Vec<Vec<(u32, Option<(i32, i32)>)>>;

pub struct Input {
    start: (i32, i32),
    end: (i32, i32),
    map: Vec<Vec<bool>>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let m = input
        .lines()
        .zip(0..)
        .map(|(line, i)| {
            line.chars()
                .zip(0..)
                .map(|(c, j)| {
                    let mut tile = true;
                    match c {
                        'S' => start = (i, j),
                        'E' => end = (i, j),
                        '#' => tile = false,
                        _ => {}
                    }
                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Input { start, end, map: m }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> u32 {
    let cost_map = build_cost_map(input);
    cost_map[input.end.0 as usize][input.end.1 as usize].0
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> u32 {
    let mut cost_map = build_cost_map(input);
    let mut stack = vec![input.end];
    let mut visited = HashSet::new();

    while let Some(current) = stack.pop() {
        if !visited.contains(&current) {
            visited.insert(current);
            if current != input.start {
                if let Some(current_step) = cost_map[current.0 as usize][current.1 as usize].1 {
                    let current_cost = cost_map[current.0 as usize][current.1 as usize].0;
                    let prev = (current.0 - current_step.0, current.1 - current_step.1);
                    let prev_cost = cost_map[prev.0 as usize][prev.1 as usize].0;
                    if prev_cost != current_cost - 1 && prev_cost != current_cost - 1001 {
                        cost_map[prev.0 as usize][prev.1 as usize] =
                            (current_cost - 1, Some(current_step));
                    }
                    stack.push(prev);
                    let double_step = (prev.0 - current_step.0, prev.1 - current_step.1);
                    if cost_map[double_step.0 as usize][double_step.1 as usize].0
                        == current_cost - 2
                    {
                        stack.push(double_step);
                    }
                }
            }
        }
    }

    visited.len() as u32
}

fn build_cost_map(input: &Input) -> CostMap {
    let start_dir = (1, 0);
    let mut cost_map = vec![vec![(u32::MAX, None); input.map[0].len()]; input.map.len()];
    cost_map[input.start.0 as usize][input.start.1 as usize] = (0, Some(start_dir));

    let mut deque = VecDeque::from([(input.start, start_dir)]);
    while let Some((current, current_direction)) = deque.pop_front() {
        let (current_cost, direction_via_cost) = cost_map[current.0 as usize][current.1 as usize];
        let actual_direction = direction_via_cost.unwrap_or(current_direction);
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter(|(dx, dy)| (-1 * dx, -1 * dy) != actual_direction)
            .for_each(|direction| {
                let new = (current.0 + direction.0, current.1 + direction.1);
                let new_cost = if *direction == actual_direction {
                    current_cost + 1
                } else {
                    current_cost + 1001
                };
                if input.map[new.0 as usize][new.1 as usize]
                    && new_cost < cost_map[new.0 as usize][new.1 as usize].0
                {
                    cost_map[new.0 as usize][new.1 as usize] = (new_cost, Some(*direction));
                    deque.push_back((new, *direction));
                }
            });
    }
    cost_map
}
