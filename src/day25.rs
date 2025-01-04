use itertools::Itertools;

#[derive(Debug)]
pub struct Key {
    pins: Vec<u8>,
}

#[derive(Debug)]
pub struct Lock {
    pins: Vec<u8>,
    size: u8,
}

#[derive(Debug)]
pub struct Input {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Input {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    input.split("\n\n").for_each(|group| {
        let mut is_key = None;
        let mut pins = Vec::new();
        let mut size = 0;
        for line in group.lines() {
            size += 1;
            if is_key.is_none() {
                is_key = Some(line.chars().all_equal_value() == Ok('.'));
                pins.resize(line.len(), 0);
            }
            line.char_indices().for_each(|(i, c)| {
                if c == '#' {
                    pins[i] += 1;
                }
            });
        }

        pins.iter_mut().for_each(|p| {
            *p -= 1;
        });

        match is_key {
            Some(true) => keys.push(Key { pins }),
            Some(false) => locks.push(Lock { pins, size }),
            _ => {}
        }
    });

    Input { keys, locks }
}

#[aoc(day25, part1)]
pub fn part1(input: &Input) -> u64 {
    let mut count = 0;
    input.keys.iter().for_each(|key| {
        input.locks.iter().for_each(|lock| {
            if check_key(key, lock) {
                count += 1;
            }
        });
    });
    count
}

fn check_key(key: &Key, lock: &Lock) -> bool {
    key.pins
        .iter()
        .zip(lock.pins.iter())
        .all(|(k, l)| k + l < lock.size - 1)
}
