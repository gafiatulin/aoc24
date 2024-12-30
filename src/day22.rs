use std::collections::{HashMap, HashSet};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<u128> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &[u128]) -> u128 {
    input.iter().map(|&n| steps(n, 2000)[2000]).sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &[u128]) -> u128 {
    let maps = input
        .iter()
        .map(|&n| change_seq_to_price(&steps(n, 2000)))
        .collect::<Vec<_>>();
    let keys = maps.iter().fold(HashSet::new(), |acc, map| {
        acc.union(&map.keys().collect()).cloned().collect()
    });
    keys.iter()
        .map(|key| maps.iter().map(|map| map.get(*key).unwrap_or(&0)).sum())
        .max()
        .unwrap()
}

fn steps(n: u128, steps: u32) -> Vec<u128> {
    let mut vec = vec![n; steps as usize + 1];
    let mut n = n;
    for i in 1..=steps {
        n = step(n);
        vec[i as usize] = n;
    }
    vec
}

fn step(num: u128) -> u128 {
    let mut num = num;
    num = (num ^ (num << 6)) % 16777216;
    num = (num ^ (num >> 5)) % 16777216;
    (num ^ (num << 11)) % 16777216
}

fn change_seq_to_price(sns: &[u128]) -> HashMap<[i8; 4], u128> {
    let changes = sns
        .iter()
        .zip(sns.iter().skip(1))
        .map(|(&a, &b)| ((b % 10) as i8 - (a % 10) as i8, b % 10))
        .collect::<Vec<_>>();

    let size = changes.len();

    let mut map = HashMap::new();
    for i in 3..size {
        let key = [
            changes[i - 3].0,
            changes[i - 2].0,
            changes[i - 1].0,
            changes[i].0,
        ];
        let value = changes[i].1;
        map.entry(key).or_insert(value);
    }
    map
}
