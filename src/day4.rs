use std::collections::HashMap;

pub struct Input {
    m: HashMap<(i32, i32), char>,
    vx: Vec<(i32, i32)>,
    va: Vec<(i32, i32)>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let (m, vx, va) = input.lines().zip(0..).fold(
        (HashMap::new(), Vec::new(), Vec::new()),
        |(m, vx, va), (line, i)| {
            line.chars()
                .zip(0..)
                .fold((m, vx, va), |(mut mm, mut vvx, mut vva), (c, j)| {
                    mm.insert((i, j), c);
                    if c == 'X' {
                        vvx.push((i, j))
                    };
                    if c == 'A' {
                        vva.push((i, j))
                    };
                    (mm, vvx, vva)
                })
        },
    );

    Input { m, vx, va }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> u32 {
    input
        .vx
        .iter()
        .fold(0, |acc, (i, j)| acc + count_xmas(&input.m, *i, *j))
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> u32 {
    input
        .va
        .iter()
        .fold(0, |acc, (i, j)| acc + count_mas(&input.m, *i, *j))
}

fn count_xmas(m: &HashMap<(i32, i32), char>, i: i32, j: i32) -> u32 {
    let pattern = "XMAS";
    (-1..2)
        .flat_map(|i| (-1..2).map(move |j| (i, j)))
        .fold(0, |acc, (dx, dy)| {
            if dx == 0 && dy == 0 {
                return acc;
            };
            let mut matches = true;
            let mut step: usize = 1;
            while step < 4 && matches {
                let x = i + step as i32 * dx;
                let y = j + step as i32 * dy;
                matches = m
                    .get(&(x, y))
                    .is_some_and(|c| *c == pattern.chars().nth(step).unwrap());
                step += 1;
            }
            if matches { acc + 1 } else { acc }
        })
}

fn count_mas(m: &HashMap<(i32, i32), char>, i: i32, j: i32) -> u32 {
    let check_d = |a: &(i32, i32), b: &(i32, i32)| -> bool {
        let s: String = [m.get(a), m.get(b)]
            .map(|c| *c.unwrap_or(&' '))
            .iter()
            .collect();
        s == "MS" || s == "SM"
    };

    if check_d(&(i - 1, j - 1), &(i + 1, j + 1)) && check_d(&(i - 1, j + 1), &(i + 1, j - 1)) {
        1
    } else {
        0
    }
}
