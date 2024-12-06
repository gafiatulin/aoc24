pub struct Input {
    aas: Vec<u64>,
    bbs: Vec<u64>,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let parsed = input.lines().flat_map(|line| {
        line.split_once("   ")
            .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
    });

    let (mut aas, mut bbs): (Vec<_>, Vec<_>) = parsed.unzip();
    aas.sort();
    bbs.sort();
    Input { aas, bbs }
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> u64 {
    input
        .aas
        .iter()
        .zip(input.bbs.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &Input) -> u64 {
    let b_counts = input
        .bbs
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        });

    input
        .aas
        .iter()
        .fold(0, |acc, a| acc + a * b_counts.get(&a).unwrap_or(&0))
}
