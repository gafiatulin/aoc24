use std::collections::VecDeque;

pub fn mk_dist_map(
    map: &[Vec<bool>],
    start: (usize, usize),
    dimensions: (usize, usize),
) -> Vec<Vec<u64>> {
    let mut dist_map = vec![vec![u64::MAX; dimensions.1]; dimensions.0];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist_map[start.0][start.1] = 0;

    while let Some((y, x)) = queue.pop_front() {
        let dist = dist_map[y][x];
        if x > 0 && map[y][x - 1] && dist + 1 < dist_map[y][x - 1] {
            dist_map[y][x - 1] = dist + 1;
            queue.push_back((y, x - 1));
        }
        if x < dimensions.1 - 1 && map[y][x + 1] && dist + 1 < dist_map[y][x + 1] {
            dist_map[y][x + 1] = dist + 1;
            queue.push_back((y, x + 1));
        }
        if y > 0 && map[y - 1][x] && dist + 1 < dist_map[y - 1][x] {
            dist_map[y - 1][x] = dist + 1;
            queue.push_back((y - 1, x));
        }
        if y < dimensions.0 - 1 && map[y + 1][x] && dist + 1 < dist_map[y + 1][x] {
            dist_map[y + 1][x] = dist + 1;
            queue.push_back((y + 1, x));
        }
    }

    dist_map
}
