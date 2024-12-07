use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone)]
enum Operator {
    Add,
    Mul,
    Concat,
}

pub struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Equation> {
    input
        .lines()
        .flat_map(|line| {
            line.split_once(": ").and_then(|(result, operands)| {
                let result = result.parse().ok()?;
                let operands = operands.split(" ").flat_map(|x| x.parse().ok()).collect();
                Some(Equation { result, operands })
            })
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Equation]) -> u64 {
    part(input, false)
}

#[aoc(day7, part2)]
pub fn part2(input: &[Equation]) -> u64 {
    part(input, true)
}

fn part(input: &[Equation], has_concat: bool) -> u64 {
    input
        .par_iter()
        .filter(|eq| check_eq(eq, has_concat))
        .map(|eq| eq.result)
        .sum()
}

fn check_eq(eq: &Equation, has_concat: bool) -> bool {
    let ops = if has_concat {
        vec![Operator::Add, Operator::Mul, Operator::Concat]
    } else {
        vec![Operator::Add, Operator::Mul]
    };

    (0..eq.operands.len() - 1)
        .map(|_| ops.clone().into_iter())
        .multi_cartesian_product()
        .map(|operator_seq| eval(&operator_seq, &eq.operands))
        .any(|x| x == eq.result)
}

fn eval(operator_seq: &[Operator], operands: &[u64]) -> u64 {
    if operands.is_empty() {
        0
    } else if operands.len() == 1 {
        operands[0]
    } else {
        operands
            .iter()
            .skip(1)
            .zip(operator_seq)
            .fold(operands[0], |acc, (operand, op)| match op {
                Operator::Add => acc + operand,
                Operator::Mul => acc * operand,
                Operator::Concat => {
                    let mut acc = acc.to_string();
                    acc.push_str(&operand.to_string());
                    acc.parse().unwrap()
                }
            })
    }
}
