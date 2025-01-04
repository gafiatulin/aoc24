use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

pub struct Input {
    gates: HashMap<String, Gate>,
    wires: HashMap<String, bool>,
    zs: HashSet<String>,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Input {
    let mut gates = HashMap::new();
    let mut wires = HashMap::new();
    let mut zs: HashSet<String> = HashSet::new();
    input.lines().for_each(|line| {
        if let Some((label, v)) = line.split_once(":") {
            wires.insert(label.to_string(), v.trim() == "1");
            if label.starts_with("z") {
                zs.insert(label.to_string());
            }
        } else if let Some((op, res)) = line.split_once("->") {
            let res = res.trim();
            if let Some((a, op, b)) = op.split_whitespace().collect_tuple() {
                match op {
                    "AND" => gates.insert(res.to_string(), Gate::And(a.to_string(), b.to_string())),
                    "OR" => gates.insert(res.to_string(), Gate::Or(a.to_string(), b.to_string())),
                    "XOR" => gates.insert(res.to_string(), Gate::Xor(a.to_string(), b.to_string())),
                    _ => None,
                };

                [res, a, b].iter().for_each(|x| {
                    if x.starts_with("z") {
                        zs.insert(x.to_string());
                    }
                });
            }
        }
    });

    Input { gates, wires, zs }
}

#[aoc(day24, part1)]
pub fn part1(input: &Input) -> u64 {
    let mut res = input.wires.clone();
    input.zs.iter().for_each(|output| {
        resolve(output, &input.gates, &mut res);
    });

    to_num(&input.zs, &res)
}

#[aoc(day24, part2)]
pub fn part2(input: &Input) -> String {
    let mut misplaced = HashSet::new();

    let or_read_sources = input
        .gates
        .iter()
        .flat_map(|(_, g)| {
            if let Gate::Or(a, b) = g {
                vec![a.clone(), b.clone()]
            } else {
                Vec::new()
            }
        })
        .collect::<HashSet<String>>();

    input.gates.iter().for_each(|(l, g)| {
        if l.starts_with("z") && l != "z45" && !matches!(g, Gate::Xor(_, _)) {
            misplaced.insert(l.to_string());
        }
        if or_read_sources.contains(l) && !matches!(g, Gate::And(_, _)) {
            misplaced.insert(l.to_string());
        }
        match g {
            Gate::Xor(a, b) => {
                let writes_to_output = l.starts_with("z");
                let reads_from_inputs = (a.starts_with("x") && b.starts_with("y"))
                    || (a.starts_with("y") && b.starts_with("x"));
                match (writes_to_output, reads_from_inputs) {
                    (false, false) => {
                        misplaced.insert(l.to_string());
                    }
                    (true, true) if l != "z00" => {
                        misplaced.insert(l.to_string());
                    }
                    _ => {}
                }
            }
            Gate::And(a, b) => {
                let reads_from_first_inputs =
                    (a == "x00" && b == "y00") || (a == "y00" && b == "x00");
                let writes_to_or = or_read_sources.contains(l);
                if !(writes_to_or || reads_from_first_inputs) {
                    misplaced.insert(l.to_string());
                }
            }
            _ => {}
        }
    });

    misplaced.iter().sorted().join(",")
}

fn resolve(output: &str, gates: &HashMap<String, Gate>, res: &mut HashMap<String, bool>) -> bool {
    if let Some(&value) = res.get(output) {
        value
    } else {
        let value = match gates.get(output).unwrap() {
            Gate::And(a, b) => resolve(a, gates, res) & resolve(b, gates, res),
            Gate::Or(a, b) => resolve(a, gates, res) | resolve(b, gates, res),
            Gate::Xor(a, b) => resolve(a, gates, res) ^ resolve(b, gates, res),
        };
        res.insert(output.to_string(), value);
        value
    }
}

fn to_num(labels: &HashSet<String>, wires: &HashMap<String, bool>) -> u64 {
    labels.iter().sorted().zip(0..).fold(0, |acc, (output, i)| {
        acc + (wires[output] as u64) * 2u64.pow(i)
    })
}
