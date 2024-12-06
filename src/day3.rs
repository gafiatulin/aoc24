#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    solve(input).0
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    solve(input).1
}

fn solve(input: &str) -> (u64, u64) {
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut enabled = true;
    while i < chars.len() - 6 {
        if &input[i..i + 4] == "mul(" {
            let x = &input[i + 4..i + 7]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            let a = x.parse::<u64>().unwrap();
            if a > 0 && a < 1000 && chars[i + 3 + x.len() + 1] == ',' {
                let y = &input[i + 3 + x.len() + 2..i + 3 + x.len() + 5]
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>();
                let b = y.parse::<u64>().unwrap();
                if b > 0 && b < 1000 && chars[i + x.len() + y.len() + 5] == ')' {
                    p1 += a * b;
                    p2 += if enabled { a * b } else { 0 };
                    i += x.len() + y.len() + 6;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        } else if &input[i..i + 4] == "do()" {
            enabled = true;
            i += 4;
        } else if &input[i..i + 7] == "don't()" {
            enabled = false;
            i += 7;
        } else {
            i += 1;
        }
    }
    (p1, p2)
}
