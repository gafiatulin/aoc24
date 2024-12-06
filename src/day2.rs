#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<i64>]) -> u64 {
    input.iter().filter(|row| check_safe(row)).count() as u64
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<i64>]) -> u64 {
    input.iter().fold(0, |acc, row| {
        if check_safe(row) {
            acc + 1
        } else {
            let mut i = 0;
            let mut found = false;
            while i < row.len() && !found {
                let mut new_row = row.clone();
                new_row.remove(i);
                if check_safe(&new_row) {
                    found = true;
                } else {
                    i += 1;
                }
            }
            if found { acc + 1 } else { acc }
        }
    })
}

fn check_safe(row: &[i64]) -> bool {
    let first = row[0];
    let second = row[1];
    let sign: i64 = if (second - first) > 0 { 1 } else { -1 };

    let mut safe = true;
    let mut i = 1;
    while i < row.len() && safe {
        let diff = (row[i] - row[i - 1]) * sign;
        safe = (1..=3).contains(&diff);
        i += 1;
    }
    safe
}
