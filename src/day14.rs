use bmp::{Image, Pixel};
use std::fs;

const IMG_DIR: &str = "output/2024/day14/";
const DIM_X: i32 = 101;
const DIM_Y: i32 = 103;

#[derive(Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Robot> {
    input
        .lines()
        .flat_map(|line| match line.split_once(" ") {
            Some((p, v)) => {
                let pos = p.strip_prefix("p=").and_then(parse_point);
                let vel = v.strip_prefix("v=").and_then(parse_point);
                pos.zip(vel).map(|((pos_x, pos_y), (vel_x, vel_y))| Robot {
                    x: pos_x,
                    y: pos_y,
                    vel_x,
                    vel_y,
                })
            }
            _ => None,
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Robot]) -> u32 {
    let mut rs = input.to_vec();
    let mut count = 0;
    while count < 100 {
        step(&mut rs);
        count += 1;
    }

    let (q1, q2, q3, q4) = rs
        .into_iter()
        .fold((0, 0, 0, 0), |quad, r| match (r.x, r.y) {
            (x, y) if x < DIM_X / 2 && y < DIM_Y / 2 => (quad.0 + 1, quad.1, quad.2, quad.3),
            (x, y) if x > DIM_X / 2 && y < DIM_Y / 2 => (quad.0, quad.1 + 1, quad.2, quad.3),
            (x, y) if x < DIM_X / 2 && y > DIM_Y / 2 => (quad.0, quad.1, quad.2 + 1, quad.3),
            (x, y) if x > DIM_X / 2 && y > DIM_Y / 2 => (quad.0, quad.1, quad.2, quad.3 + 1),
            _ => quad,
        });

    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
pub fn part2(input: &[Robot]) -> u32 {
    let render = false;
    if render {
        ensure_dir();
        let mut count = 0;
        let mut rs = input.to_vec();
        let cycle = DIM_X * DIM_Y;
        while count < cycle {
            render_img(&rs, count);
            step(&mut rs);
            count += 1;
        }
    }
    // After looking through the rendered images
    7037
}

fn parse_point(s: &str) -> Option<(i32, i32)> {
    let (x, y) = s.split_once(",")?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

fn step(robots: &mut [Robot]) {
    robots.iter_mut().for_each(|r| {
        r.x = (((r.x + r.vel_x) % DIM_X) + DIM_X) % DIM_X;
        r.y = (((r.y + r.vel_y) % DIM_Y) + DIM_Y) % DIM_Y;
    });
}

fn ensure_dir() {
    let pwd = std::env::current_dir().unwrap().display().to_string();
    if fs::metadata(IMG_DIR).is_err() {
        fs::create_dir_all(IMG_DIR).unwrap();
    }
    println!("Using directory: {}/{}", pwd, IMG_DIR);
}

fn render_img(robots: &[Robot], idx: i32) {
    let mut img = Image::new(DIM_X as u32, DIM_X as u32);

    for r in robots {
        img.set_pixel(r.x as u32, r.y as u32, Pixel::new(255, 255, 255));
    }

    img.save(format!("{}{}.bmp", IMG_DIR, idx)).unwrap();
}
