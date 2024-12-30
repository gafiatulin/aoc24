use crate::util::mk_dist_map;

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
pub fn part1(input: &[(usize, usize)]) -> u64 {
    path_length(input, 1024)
}

#[aoc(day18, part2)]
pub fn part2(input: &[(usize, usize)]) -> String {
    let idx = (1024..)
        .find(|steps| path_length(input, *steps) == u64::MAX)
        .unwrap();
    let (x, y) = input[idx - 1];
    format!("{},{}", x, y)
}

fn path_length(input: &[(usize, usize)], steps: usize) -> u64 {
    let dim = 71;
    let mut map = vec![vec![true; dim]; dim];

    input.iter().take(steps).for_each(|(x, y)| {
        map[*y][*x] = false;
    });

    let dist_map = mk_dist_map(&map, (0, 0), (dim, dim));
    dist_map[dim - 1][dim - 1]
}
