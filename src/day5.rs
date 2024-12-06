pub struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let rules: Vec<(u32, u32)> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .flat_map(|l| l.split_once('|'))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<_>>();

    let updates: Vec<Vec<u32>> = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    Input { rules, updates }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> u32 {
    input.updates.iter().fold(0, |acc, update| {
        if check_update(update, &input.rules) {
            acc + update[update.len() / 2]
        } else {
            acc
        }
    })
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> u32 {
    input.updates.iter().fold(0, |acc, update| {
        if check_update(update, &input.rules) {
            acc
        } else {
            let corrected = correct_update(update, &input.rules);
            acc + corrected[corrected.len() / 2]
        }
    })
}

fn check_update(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let mut i = 0;
    let mut valid = true;
    while i < rules.len() && valid {
        let (a, b) = rules[i];
        let index_of_a = update.iter().position(|&x| x == a);
        let index_of_b = update.iter().position(|&x| x == b);
        valid = match (index_of_a, index_of_b) {
            (Some(index_of_a), Some(index_of_b)) => index_of_a < index_of_b,
            _ => true,
        };
        i += 1;
    }
    valid
}

fn correct_update(update: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut corrected = update.to_owned();
    corrected.sort_by(|a, b| {
        let matching_rule: Vec<&(u32, u32)> = rules
            .iter()
            .filter(|(x, y)| (x == a && y == b) || (x == b && y == a))
            .collect();
        if matching_rule.is_empty() {
            std::cmp::Ordering::Equal
        } else {
            let (x, y) = matching_rule[0];
            if x == a && y == b {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    });
    corrected
}
