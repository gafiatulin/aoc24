use std::collections::HashSet;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone)]
pub enum Tile {
    Empty,
    Wall,
    Box(Option<bool>),
}
pub struct Input {
    map: Vec<Vec<Tile>>,
    moves: Vec<Direction>,
    position: (i32, i32),
}

impl Input {
    fn widen(&self) -> Self {
        let new_map = self.map.iter().fold(Vec::new(), |mut acc, row| {
            let new_row = row.iter().fold(Vec::new(), |mut acc, tile| {
                match tile {
                    Tile::Empty => {
                        acc.push(Tile::Empty);
                        acc.push(Tile::Empty);
                    }
                    Tile::Wall => {
                        acc.push(Tile::Wall);
                        acc.push(Tile::Wall);
                    }
                    Tile::Box(_) => {
                        acc.push(Tile::Box(Some(true)));
                        acc.push(Tile::Box(Some(false)));
                    }
                }
                acc
            });
            acc.push(new_row);
            acc
        });

        Input {
            map: new_map,
            moves: self.moves.clone(),
            position: (self.position.0, self.position.1 * 2),
        }
    }
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Input {
    let mut pos = (0, 0);
    let map = input
        .lines()
        .take_while(|line| !line.is_empty())
        .zip(0..)
        .fold(Vec::new(), |mut acc, (line, y)| {
            let row = line.chars().zip(0..).fold(Vec::new(), |mut acc, (c, x)| {
                match c {
                    '#' => acc.push(Tile::Wall),
                    'O' => acc.push(Tile::Box(None)),
                    '@' => {
                        pos = (y, x);
                        acc.push(Tile::Empty);
                    }
                    _ => acc.push(Tile::Empty),
                }
                acc
            });
            acc.push(row);
            acc
        });

    let moves = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '>' => Direction::Right,
                _ => Direction::Left,
            })
        })
        .collect();
    Input {
        map,
        moves,
        position: pos,
    }
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> u32 {
    part(input)
}

#[aoc(day15, part2)]
fn part2(input: &Input) -> u32 {
    part(&input.widen())
}

fn part(input: &Input) -> u32 {
    let (mut pos_y, mut pos_x) = input.position;
    let mut map = input.map.clone();
    input.moves.iter().for_each(|m| {
        (pos_y, pos_x) = make_move(&mut map, pos_y, pos_x, m);
    });

    map.iter().zip(0..).fold(0, |acc, (row, i)| {
        row.iter().zip(0..).fold(acc, |acc, (tile, j)| match tile {
            Tile::Box(part) if part.unwrap_or(true) => acc + 100 * i + j,
            _ => acc,
        })
    })
}

fn make_move(map: &mut [Vec<Tile>], pos_y: i32, pos_x: i32, m: &Direction) -> (i32, i32) {
    let (dy, dx) = m.move_delta();
    match map[(pos_y + dy) as usize][(pos_x + dx) as usize] {
        Tile::Empty => (pos_y + dy, pos_x + dx),
        Tile::Wall => (pos_y, pos_x),
        Tile::Box(_) => {
            let mut can_move = true;
            let mut to_be_moved: HashSet<(i32, i32)> = HashSet::from([(pos_y + dy, pos_x + dx)]);
            let mut stack = vec![(pos_y + dy, pos_x + dx)];
            while can_move && !stack.is_empty() {
                if let Some((y, x)) = stack.pop() {
                    match map[y as usize][x as usize] {
                        Tile::Empty => {}
                        Tile::Wall => {
                            can_move = false;
                        }
                        Tile::Box(part) => {
                            to_be_moved.insert((y, x));
                            stack.push((y + dy, x + dx));

                            if let Some(part) =
                                part.map(|p| if p { (y, x + 1) } else { (y, x - 1) })
                            {
                                if !to_be_moved.contains(&part) {
                                    to_be_moved.insert(part);
                                    stack.push(part);
                                }
                            }
                        }
                    }
                }
            }

            if can_move {
                let entries: Vec<(i32, i32, Tile)> = to_be_moved
                    .iter()
                    .map(|(y, x)| (*y, *x, map[*y as usize][*x as usize].clone()))
                    .collect();

                to_be_moved.iter().for_each(|(y, x)| {
                    map[*y as usize][*x as usize] = Tile::Empty;
                });

                entries.iter().for_each(|(y, x, t)| {
                    map[(*y + dy) as usize][(*x + dx) as usize] = t.clone();
                });

                (pos_y + dy, pos_x + dx)
            } else {
                (pos_y, pos_x)
            }
        }
    }
}
