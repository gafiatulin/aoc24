use std::collections::HashMap;
use std::iter::from_fn;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Stone {
    number: Vec<u8>,
}

impl Stone {
    pub fn as_number(&self) -> u128 {
        self.number.iter().fold(0, |acc, x| acc * 10 + *x as u128)
    }

    pub fn from_number(mut number: u128) -> Stone {
        let num = from_fn(move || {
            if number == 0 {
                None
            } else {
                let digit = (number % 10) as u8;
                number /= 10;
                Some(digit)
            }
        })
        .collect::<Vec<u8>>()
        .iter()
        .rev()
        .copied()
        .collect();

        Stone { number: num }
    }

    pub fn blink(self) -> Vec<Stone> {
        let mut stones = Vec::new();
        match self.number.as_slice() {
            [0] => stones.push(Stone { number: vec![1] }),
            v if v.len() % 2 == 0 => {
                let (first_half, second_half) = v.split_at(v.len() / 2);
                stones.push(Stone {
                    number: first_half.to_vec(),
                });
                let reduced: Vec<u8> = second_half
                    .iter()
                    .skip_while(|x| *x == &0)
                    .copied()
                    .collect();
                let second = if reduced.is_empty() { vec![0] } else { reduced };
                stones.push(Stone { number: second });
            }
            _ => stones.push(Stone::from_number(Stone::as_number(&self) * 2024)),
        }
        stones
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Stone> {
    input
        .split(' ')
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .map(|v| Stone { number: v })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &[Stone]) -> u64 {
    count_after_blinks(input, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &[Stone]) -> u64 {
    count_after_blinks(input, 75)
}

fn count_after_blinks(input: &[Stone], blinks: u8) -> u64 {
    let mut mem: HashMap<(u8, Stone), u64> = HashMap::new();
    input
        .iter()
        .map(|stone| count(stone, blinks, &mut mem))
        .sum()
}

fn count(stone: &Stone, blinks: u8, mem: &mut HashMap<(u8, Stone), u64>) -> u64 {
    if let Some(&count) = mem.get(&(blinks, stone.clone())) {
        count
    } else if blinks == 0 {
        1
    } else {
        let count = stone
            .clone()
            .blink()
            .iter()
            .map(|s| count(s, blinks - 1, mem))
            .sum();
        mem.insert((blinks, stone.clone()), count);
        count
    }
}
