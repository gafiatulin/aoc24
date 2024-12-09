#[derive(Clone)]
pub struct File {
    idx: u32,
    size: u32,
    offset: u32,
}

#[derive(Default)]
pub struct DiskMap {
    layout: Vec<i32>,
    files: Vec<File>,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> DiskMap {
    input
        .chars()
        .fold((DiskMap::default(), 0, 0), |(mut dm, count, offset), c| {
            if let Some(d) = c.to_digit(10) {
                let idx = count / 2;
                let rem = count % 2;
                if rem == 0 {
                    dm.layout.append(&mut vec![idx; d as usize]);
                    dm.files.push(File {
                        idx: idx as u32,
                        size: d,
                        offset,
                    });
                } else {
                    dm.layout.append(&mut vec![-1; d as usize]);
                }
                (dm, count + 1, offset + d)
            } else {
                (dm, count, offset)
            }
        })
        .0
}

#[aoc(day9, part1)]
pub fn part1(input: &DiskMap) -> u64 {
    let mut i = 0;
    let mut j = input.layout.len() - 1;
    let compressed = &mut input.layout.to_vec();

    while i < j {
        if compressed[i] >= 0 {
            i += 1;
        } else if compressed[j] >= 0 {
            compressed[i] = compressed[j];
            compressed[j] = -1;
            j -= 1;
        } else {
            j -= 1;
        }
    }

    compressed.iter().zip(0..).fold(0, |acc, (x, i)| {
        if *x >= 0 {
            acc + (i as u64 * *x as u64)
        } else {
            acc
        }
    })
}

#[aoc(day9, part2)]
pub fn part2(input: &DiskMap) -> u64 {
    let mut i = 0;
    let mut j = input.files.len() - 1;
    let mut compressed = input.files.to_vec();

    while j > 0 && i <= j {
        let curr = &compressed[j].clone();
        let before_gap = &compressed[i];
        let after_gap = &compressed[i + 1];
        let gap = after_gap.offset - before_gap.offset - before_gap.size;
        if gap >= curr.size {
            let mut new_compressed = compressed[..=i].to_vec();
            new_compressed.push(File {
                idx: curr.idx,
                size: curr.size,
                offset: before_gap.offset + before_gap.size,
            });
            new_compressed.append(&mut compressed[i + 1..j].to_vec());
            new_compressed.append(&mut compressed[j + 1..].to_vec());
            compressed = new_compressed.clone();
            i = 0;
        } else {
            i += 1;
        }
        if i == j {
            j -= 1;
            i = 0;
        }
    }

    compressed.iter().fold(0, |acc, file| {
        acc + file.idx as u64
            * ((file.size as u64 * (2 * file.offset as u64 + file.size as u64 - 1)) / 2)
    })
}
