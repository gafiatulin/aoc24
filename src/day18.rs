use std::collections::VecDeque;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .flat_map(|line| {
            if let Some((x, y)) = line.split_once(',') {
                Some((x.parse().unwrap(), y.parse().unwrap()))
            } else {
                None
            }
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[(usize, usize)]) -> u32 {
    path_length(input, 1024)
}

#[aoc(day18, part2)]
pub fn part2(input: &[(usize, usize)]) -> String {
    let idx = (1024..)
        .find(|steps| path_length(input, *steps) == u32::MAX)
        .unwrap();
    let (x, y) = input[idx - 1];
    format!("{},{}", x, y)
}

fn path_length(input: &[(usize, usize)], steps: usize) -> u32 {
    let dim = 71;
    let mut map = vec![vec![true; dim]; dim];
    let mut dist_map = vec![vec![u32::MAX; dim]; dim];

    input.iter().take(steps).for_each(|(x, y)| {
        map[*y][*x] = false;
    });

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    dist_map[0][0] = 0;
    while let Some((x, y)) = queue.pop_front() {
        let dist = dist_map[y][x];
        if x > 0 && map[y][x - 1] && dist + 1 < dist_map[y][x - 1] {
            dist_map[y][x - 1] = dist + 1;
            queue.push_back((x - 1, y));
        }
        if x < dim - 1 && map[y][x + 1] && dist + 1 < dist_map[y][x + 1] {
            dist_map[y][x + 1] = dist + 1;
            queue.push_back((x + 1, y));
        }
        if y > 0 && map[y - 1][x] && dist + 1 < dist_map[y - 1][x] {
            dist_map[y - 1][x] = dist + 1;
            queue.push_back((x, y - 1));
        }
        if y < dim - 1 && map[y + 1][x] && dist + 1 < dist_map[y + 1][x] {
            dist_map[y + 1][x] = dist + 1;
            queue.push_back((x, y + 1));
        }
    }

    dist_map[dim - 1][dim - 1]
}
