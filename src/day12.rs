use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct LandPlot {
    label: char,
    idx: u32,
}

#[derive(Default)]
pub struct LandPlotMap {
    plots: HashMap<(i32, i32), LandPlot>,
    index_map: HashMap<u32, HashSet<(i32, i32)>>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HashMap<u32, HashSet<(i32, i32)>> {
    input
        .lines()
        .zip(0..)
        .fold((LandPlotMap::default(), 0), |(map, idx), (l, i)| {
            l.char_indices().fold((map, idx), |(mut map, idx), (j, c)| {
                let label = c;
                let x = i;
                let y = j as i32;
                let neighbor_idx = |neighbor: &LandPlot| {
                    if neighbor.label == label {
                        Some(neighbor.idx)
                    } else {
                        None
                    }
                };
                let label_up = map.plots.get(&(x - 1, y)).and_then(neighbor_idx);
                let label_left = map.plots.get(&(x, y - 1)).and_then(neighbor_idx);
                let next_idx = match (label_up, label_left) {
                    (None, None) => {
                        map.plots.insert((x, y), LandPlot { label, idx });
                        let mut hs = HashSet::new();
                        hs.insert((x, y));
                        map.index_map.insert(idx, hs);
                        idx + 1
                    }
                    (Some(a), Some(b)) => {
                        if a == b {
                            map.plots.insert((x, y), LandPlot { label, idx: a });
                            map.index_map.get_mut(&a).unwrap().insert((x, y));
                        } else {
                            let plot = LandPlot { label, idx: a };
                            let bs = map.index_map.remove(&b).unwrap();
                            map.plots.insert((x, y), plot);
                            bs.iter().for_each(|&(i, j)| {
                                map.plots.get_mut(&(i, j)).unwrap().idx = a;
                            });
                            let im_a = map.index_map.get_mut(&a).unwrap();
                            im_a.extend(bs);
                            im_a.insert((x, y));
                        }
                        idx
                    }
                    (a, b) => {
                        let taken_idx = a.or(b).unwrap();
                        map.plots.insert((x, y), LandPlot {
                            label,
                            idx: taken_idx,
                        });
                        map.index_map.get_mut(&taken_idx).unwrap().insert((x, y));
                        idx
                    }
                };
                (map, next_idx)
            })
        })
        .0
        .index_map
}

#[aoc(day12, part1)]
pub fn part1(input: &HashMap<u32, HashSet<(i32, i32)>>) -> u32 {
    input.values().fold(0, |acc, v| {
        let area = v.len() as u32;
        let perimeter = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .fold(0, |acc, &(dx, dy)| {
                v.iter().fold(acc, |acc, (x, y)| {
                    if v.contains(&(*x + dx, *y + dy)) {
                        acc
                    } else {
                        acc + 1
                    }
                })
            });
        acc + area * perimeter
    })
}

#[aoc(day12, part2)]
pub fn part2(input: &HashMap<u32, HashSet<(i32, i32)>>) -> u32 {
    input.values().fold(0, |acc, v| {
        let area = v.len() as u32;
        let side_count = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .fold(0, |acc, &(dx, dy)| {
                v.iter()
                    .filter(move |(x, y)| !v.contains(&(*x + dx, *y + dy)))
                    .into_group_map_by(|p| if dx != 0 { p.0 } else { p.1 })
                    .values()
                    .fold(acc, |acc, v| {
                        let s = v.iter().map(|(x, y)| if dx != 0 { y } else { x }).sorted();
                        s.clone()
                            .zip(s.skip(1))
                            .fold(acc, |acc, (a, b)| if *a + 1 == *b { acc } else { acc + 1 })
                            + 1
                    })
            });

        acc + area * side_count
    })
}
