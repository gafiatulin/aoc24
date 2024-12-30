use crate::util::mk_dist_map;

pub struct Input {
    map: Vec<Vec<bool>>,
    start: (usize, usize),
    dimensions: (usize, usize),
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    let mut start = (0, 0);
    let mut dimensions = (0, 0);
    let mut map = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        let mut row = Vec::new();
        line.char_indices().for_each(|(j, c)| {
            if c == 'S' {
                start = (i, j);
            }
            if c == '#' {
                row.push(false);
            } else {
                row.push(true);
            }
            dimensions = (i, j);
        });
        map.push(row);
    });

    Input {
        map,
        start,
        dimensions: (dimensions.0 + 1, dimensions.1 + 1),
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> u32 {
    part(input, 2)
}

#[aoc(day20, part2)]
pub fn part2(input: &Input) -> u32 {
    part(input, 20)
}

fn part(input: &Input, max_cheat: i8) -> u32 {
    let dist_map = mk_dist_map(&input.map, input.start, input.dimensions);
    (0..input.dimensions.0)
        .flat_map(|i| (0..input.dimensions.1).map(move |j| (i, j)))
        .filter(|&(i, j)| input.map[i][j])
        .flat_map(|(i, j)| {
            (-max_cheat..=max_cheat).flat_map(move |dy| {
                (-max_cheat + dy.abs()..=max_cheat - dy.abs()).flat_map(move |dx| {
                    let ii = i as i32 + dy as i32;
                    let jj = j as i32 + dx as i32;
                    if ii >= 0
                        && ii < input.dimensions.0 as i32
                        && jj >= 0
                        && jj < input.dimensions.1 as i32
                        && input.map[ii as usize][jj as usize]
                    {
                        Some((i, j, dy, dx))
                    } else {
                        None
                    }
                })
            })
        })
        .filter(|&(i, j, dy, dx)| {
            let ii = (i as i32 + dy as i32) as usize;
            let jj = (j as i32 + dx as i32) as usize;
            dist_map[ii][jj] + dy.unsigned_abs() as u64 + dx.unsigned_abs() as u64 + 100
                <= dist_map[i][j]
        })
        .count() as u32
}
