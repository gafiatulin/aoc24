use std::collections::HashMap;

pub struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let patterns = input
        .lines()
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.split(',').map(|x| x.trim().to_string()))
        .collect();
    let designs = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.to_string())
        .collect();

    Input { patterns, designs }
}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> u64 {
    let mut mem = HashMap::new();

    input
        .designs
        .iter()
        .filter(|design| possible_pattern_arrangements(design, &input.patterns, &mut mem) > 0)
        .count() as u64
}

#[aoc(day19, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut mem = HashMap::new();

    input
        .designs
        .iter()
        .map(|design| possible_pattern_arrangements(design, &input.patterns, &mut mem))
        .sum()
}

fn possible_pattern_arrangements(
    design: &str,
    patterns: &[String],
    mem: &mut HashMap<String, u64>,
) -> u64 {
    if design.is_empty() {
        1
    } else if let Some(&result) = mem.get(design) {
        result
    } else {
        let result = patterns
            .iter()
            .map(|pattern| {
                if design.starts_with(pattern) {
                    possible_pattern_arrangements(
                        design.strip_prefix(pattern).unwrap(),
                        patterns,
                        mem,
                    )
                } else {
                    0
                }
            })
            .sum();

        mem.insert(design.to_string(), result);

        result
    }
}
