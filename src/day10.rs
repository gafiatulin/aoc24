use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct HeightMap {
    heights: HashMap<Point, u8>,
    points_at_height: HashMap<u8, HashSet<Point>>,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> HeightMap {
    input
        .lines()
        .zip(0..)
        .fold(HeightMap::default(), |acc, (l, i)| {
            l.char_indices().fold(acc, |mut acc, (j, c)| {
                let p = Point { x: i, y: j as i32 };
                match c.to_digit(10) {
                    None => acc,
                    Some(h) => {
                        acc.heights.insert(p, h as u8);
                        acc.points_at_height
                            .entry(h as u8)
                            .or_default()
                            .insert(p);
                        acc
                    }
                }
            })
        })
}

#[aoc(day10, part1)]
pub fn part1(hm: &HeightMap) -> u32 {
    match hm.points_at_height.get(&0) {
        None => 0,
        Some(potential_trail_heads) => {
            potential_trail_heads
                .iter()
                .map(|start| {
                    list_trail_paths(start, hm)
                        .iter()
                        .flat_map(|trail| trail.last())
                        .collect::<HashSet<&Point>>()
                        .len() as u32
                })
                .sum()
        }
    }
}

#[aoc(day10, part2)]
pub fn part2(hm: &HeightMap) -> u32 {
    match hm.points_at_height.get(&0) {
        None => 0,
        Some(potential_trail_heads) => {
            potential_trail_heads
                .iter()
                .map(|start| {
                    list_trail_paths(start, hm).len() as u32
                })
                .sum()
        }
    }
}

fn list_trail_paths(start: &Point, hm: &HeightMap) -> Vec<Vec<Point>> {
    let mut distinct_trail_paths: Vec<Vec<Point>> = Vec::new();
    let mut stack: Vec<Vec<Point>> = Vec::new();
    stack.push(vec![*start]);

    while let Some(current_path)  = stack.pop() {
        let current_point = *current_path.last().unwrap();
        let current_height = *hm.heights.get(&current_point).unwrap();
        [
            Point { x: current_point.x + 1, y: current_point.y },
            Point { x: current_point.x - 1, y: current_point.y },
            Point { x: current_point.x, y: current_point.y + 1 },
            Point { x: current_point.x, y: current_point.y - 1 },
        ].iter().filter(|p| {
            let next_height = hm.heights.get(p);
            match next_height {
                None => false,
                Some(next_height) => *next_height == current_height + 1
            }
        }).for_each(|next_point| {
            let mut new_path = current_path.clone();
            new_path.push(*next_point);
            if new_path.len() == 10 {
                distinct_trail_paths.push(new_path);
            } else {
                stack.push(new_path);
            }
        });
    }

    distinct_trail_paths
}
