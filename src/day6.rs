use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}
#[derive(Debug, Clone)]
pub struct State {
    position: Position,
    obstructions: HashSet<(i32, i32)>,
    dimensions: (i32, i32),
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: Position {
                x: 0,
                y: 0,
                direction: Direction::Up,
            },
            obstructions: HashSet::new(),
            dimensions: (0, 0),
        }
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> State {
    input
        .lines()
        .zip(0..)
        .fold(State::default(), |input, (line, i)| {
            line.chars().zip(0..).fold(input, |mut input, (c, j)| {
                input.dimensions = (i, j);
                if c == '#' {
                    input.obstructions.insert((i, j));
                } else if c == '^' {
                    input.position = Position {
                        x: i,
                        y: j,
                        direction: Direction::Up,
                    };
                }
                input
            })
        })
}

#[aoc(day6, part1)]
pub fn part1(input: &State) -> Option<u32> {
    walk(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &State) -> u32 {
    let potential_obstructions = (0..input.dimensions.0 + 1)
        .flat_map(|i| (0..input.dimensions.1 + 1).map(move |j| (i, j)))
        .filter(|(i, j)| {
            !(input.obstructions.contains(&(*i, *j))
                || (*i, *j) == (input.position.x, input.position.y))
        })
        .collect::<Vec<_>>();

    potential_obstructions
        .par_iter()
        .filter(|(i, j)| {
            let mut new_state = input.clone();
            new_state.obstructions.insert((*i, *j));
            walk(&new_state).is_none()
        })
        .count() as u32
}

fn walk(state: &State) -> Option<u32> {
    let mut p: Position = state.position;
    let (max_x, max_y) = state.dimensions;
    let mut visited = HashSet::new();
    let mut path_loop = false;
    while p.x >= 0 && p.x <= max_x && p.y >= 0 && p.y <= max_y && !path_loop {
        path_loop = !visited.insert(p);
        let step = match p.direction {
            Direction::Up => (p.x - 1, p.y),
            Direction::Down => (p.x + 1, p.y),
            Direction::Left => (p.x, p.y - 1),
            Direction::Right => (p.x, p.y + 1),
        };
        if state.obstructions.contains(&step) {
            p = Position {
                direction: match p.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
                ..p
            }
        } else {
            p = Position {
                x: step.0,
                y: step.1,
                ..p
            };
        }
    }

    if path_loop {
        None
    } else {
        let positions = visited.iter().map(|p| (p.x, p.y)).collect::<HashSet<_>>();
        Some(positions.len() as u32)
    }
}
