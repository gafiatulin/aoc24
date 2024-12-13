use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct Input {
    antenna_groups: HashMap<char, Vec<Point>>,
    dimensions: Point,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .zip(0..)
        .fold(Input::default(), |acc, (l, i)| {
            l.char_indices().fold(acc, |mut acc, (j, c)| {
                acc.dimensions = Point { x: i, y: j as i32 };
                if c != '.' {
                    acc.antenna_groups
                        .entry(c)
                        .or_default()
                        .push(acc.dimensions);
                    acc
                } else {
                    acc
                }
            })
        })
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> u32 {
    count_antinodes(input, true)
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> u32 {
    count_antinodes(input, false)
}

fn count_antinodes(input: &Input, limit: bool) -> u32 {
    input
        .antenna_groups
        .values()
        .fold(HashSet::new(), |acc, antennas| {
            acc.union(&antinodes(&input.dimensions, antennas, limit))
                .copied()
                .collect()
        })
        .len() as u32
}

fn antinodes(dimensions: &Point, antennas: &[Point], limit: bool) -> HashSet<Point> {
    antennas
        .iter()
        .combinations(2)
        .fold(HashSet::new(), |mut acc, pair| {
            let (a, b) = (pair[0], pair[1]);
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            if !limit {
                acc.insert(*a);
                acc.insert(*b);
            }
            let up = iterate_antinodes(acc, dimensions, a, &Point { x: -dx, y: -dy }, limit);
            iterate_antinodes(up, dimensions, b, &Point { x: dx, y: dy }, limit)
        })
}

fn iterate_antinodes(
    mut acc: HashSet<Point>,
    dimensions: &Point,
    p: &Point,
    d: &Point,
    limit: bool,
) -> HashSet<Point> {
    fn next(curr: &Point, d: &Point) -> Point {
        Point {
            x: curr.x + d.x,
            y: curr.y + d.y,
        }
    }
    fn in_bounds(curr: &Point, dimensions: &Point) -> bool {
        (0..=dimensions.x).contains(&curr.x) && (0..=dimensions.y).contains(&curr.y)
    }
    let mut curr = next(p, d);
    let mut resume = true;
    while in_bounds(&curr, dimensions) && resume {
        acc.insert(curr);
        curr = next(&curr, d);
        if limit {
            resume = false;
        }
    }
    acc
}
