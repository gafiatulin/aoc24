use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Graph {
    nodes: HashMap<String, HashSet<String>>,
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Graph {
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        nodes
            .entry(a.to_string())
            .or_insert(HashSet::new())
            .insert(b.to_string());
        nodes
            .entry(b.to_string())
            .or_insert(HashSet::new())
            .insert(a.to_string());
    }
    Graph { nodes }
}

#[aoc(day23, part1)]
pub fn part1(input: &Graph) -> usize {
    input
        .nodes
        .iter()
        .filter(|(k, _)| k.starts_with("t"))
        .flat_map(|(k, v)| {
            v.iter()
                .tuple_combinations()
                .filter(|(a, b)| input.nodes[*a].contains(*b))
                .map(|(a, b)| {
                    [k.to_string(), a.to_string(), b.to_string()]
                        .iter()
                        .sorted()
                        .join(",")
                })
        })
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day23, part2)]
pub fn part2(input: &Graph) -> String {
    let clique = bron_kerbosch(
        &HashSet::new(),
        &mut input.nodes.keys().cloned().collect(),
        &mut HashSet::new(),
        input,
    );
    clique.iter().sorted().join(",")
}

fn bron_kerbosch(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    graph: &Graph,
) -> HashSet<String> {
    if p.is_empty() && x.is_empty() {
        r.clone()
    } else {
        let mut max_clique = HashSet::new();

        p.clone().into_iter().for_each(|v| {
            let mut rr = r.clone();
            rr.insert(v.clone());
            let neighbors = graph.nodes[&v].clone();
            let mut pp = p.intersection(&neighbors).cloned().collect();
            let mut xx = x.intersection(&neighbors).cloned().collect();
            let clique = bron_kerbosch(&rr, &mut pp, &mut xx, graph);
            if clique.len() > max_clique.len() {
                max_clique = clique;
            }

            p.remove(&v);
            x.insert(v);
        });

        max_clique
    }
}
