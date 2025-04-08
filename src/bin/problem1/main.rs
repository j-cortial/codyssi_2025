use std::iter::once;
use std::ops::{Add, Sub};

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!(
        "The answer to part 1 is {answer1}",
        answer1 = solve_part1(&input)
    );
    println!(
        "The answer to part 2 is {answer2}",
        answer2 = solve_part2(&input)
    );
    println!(
        "The answer to part 3 is {answer3}",
        answer3 = solve_part3(&input)
    );
}

type Int = i64;

type Op = fn(Int, Int) -> Int;

type Data = (Vec<Int>, Vec<Op>);

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines().rev();
    let ops = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '+' => Add::add,
            '-' => Sub::sub,
            _ => panic!(),
        })
        .collect();
    let ints = lines.rev().map(|line| line.parse().unwrap()).collect();
    (ints, ops)
}

fn solve_part1(data: &Data) -> i64 {
    let (ints, ops) = data;
    ints.iter()
        .zip(once(&(Add::add as Op)).chain(ops.iter()))
        .fold(0, |acc, (&int, op)| op(acc, int))
}

fn solve_part2(data: &Data) -> i64 {
    let (ints, ops) = data;
    ints.iter()
        .zip(once(&(Add::add as Op)).chain(ops.iter().rev()))
        .fold(0, |acc, (&int, op)| op(acc, int))
}

fn solve_part3(data: &Data) -> i64 {
    let (ints, ops) = data;
    ints.chunks_exact(2)
        .map(|pair| pair[0] * 10 + pair[1])
        .zip(once(&(Add::add as Op)).chain(ops.iter().rev()))
        .fold(0, |acc, (int, op)| op(acc, int))
}
