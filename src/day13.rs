use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
pub struct Equation {
    da: u128,
    db: u128,
    res: u128,
}

impl Equation {
    pub fn check(&self, a: u128, b: u128) -> bool {
        a * self.da + b * self.db == self.res
    }

    pub fn adjusted(&self) -> Self {
        let mut clone = *self;
        clone.res = self.res + 10000000000000;
        clone
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<(Equation, Equation)> {
    input
        .lines()
        .chunks(4)
        .into_iter()
        .flat_map(|chunk| {
            let lines: Vec<_> = chunk.collect();
            let (da1, da2) = parse_line(lines[0])?;
            let (db1, db2) = parse_line(lines[1])?;
            let (res1, res2) = parse_line(lines[2])?;
            let e1 = Equation {
                da: da1,
                db: db1,
                res: res1,
            };
            let e2 = Equation {
                da: da2,
                db: db2,
                res: res2,
            };
            Some((e1, e2))
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[(Equation, Equation)]) -> u128 {
    part(input, false)
}

#[aoc(day13, part2)]
pub fn part2(input: &[(Equation, Equation)]) -> u128 {
    part(input, true)
}

fn parse_line(s: &str) -> Option<(u128, u128)> {
    let ss = s
        .chars()
        .skip_while(|&c| c != ':')
        .skip(1)
        .collect::<String>();
    let (a_str, b_str) = ss.split_once(",")?;
    let a = a_str.trim()[2..].parse().ok()?;
    let b = b_str.trim()[2..].parse().ok()?;
    Some((a, b))
}

fn part(input: &[(Equation, Equation)], adjust: bool) -> u128 {
    input
        .iter()
        .flat_map(|(e1, e2)| {
            if adjust {
                solve(&e1.adjusted(), &e2.adjusted())
            } else {
                solve(e1, e2)
            }
        })
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn solve(e1: &Equation, e2: &Equation) -> Option<(u128, u128)> {
    let db2_over_db1 = e2.db as f64 / e1.db as f64;
    let dividend = e2.res as f64 - db2_over_db1 * e1.res as f64;
    let divisor = e2.da as f64 - db2_over_db1 * e1.da as f64;
    let a = dividend / divisor;
    let b = (e1.res as f64 - e1.da as f64 * a) / e1.db as f64;

    let a_round = a.round() as u128;
    let b_round = b.round() as u128;

    if e1.check(a_round, b_round) && e2.check(a_round, b_round) {
        Some((a_round, b_round))
    } else {
        None
    }
}
