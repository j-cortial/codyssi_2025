use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

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

type Danger = i64;

struct Grid {
    row_count: usize,
    col_count: usize,
    data: Vec<Danger>,
}

impl Grid {
    fn row(&self, i: usize) -> impl Iterator<Item = Danger> {
        self.data[i * self.col_count..]
            .iter()
            .copied()
            .take(self.col_count)
    }

    fn col(&self, j: usize) -> impl Iterator<Item = Danger> {
        self.data[j..]
            .iter()
            .copied()
            .step_by(self.col_count)
            .take(self.row_count)
    }
}

type Idx = i32;
type Position = [Idx; 2];

const MOVES: [Position; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn add(x: Position, y: Position) -> Position {
    [x[0] + y[0], x[1] + y[1]]
}

impl Grid {
    fn danger(&self, pos: &Position) -> Option<Danger> {
        if pos[0] >= 0
            && (pos[0] as usize) < self.col_count
            && pos[1] >= 0
            && (pos[1] as usize) < self.row_count
        {
            Some(self.data[self.col_count * (pos[0] as usize) + (pos[1] as usize)])
        } else {
            None
        }
    }
}

type Data = Grid;

fn parse_input(input: &str) -> Data {
    let row_count = input.lines().count();
    let col_count = input
        .lines()
        .next()
        .map(|line| line.split_ascii_whitespace().count())
        .unwrap();
    let data = input
        .split_ascii_whitespace()
        .map(|token| token.parse().unwrap())
        .collect();
    Data {
        row_count,
        col_count,
        data,
    }
}

fn solve_part1(data: &Data) -> Danger {
    let lowest_row_danger: Danger = (0..data.row_count)
        .map(|i| data.row(i).sum())
        .min()
        .unwrap();
    let lowest_col_danger = (0..data.col_count)
        .map(|j| data.col(j).sum())
        .min()
        .unwrap();
    lowest_row_danger.min(lowest_col_danger)
}

#[derive(PartialEq, Eq, Ord)]
struct Candidate {
    position: Position,
    danger: Danger,
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.danger.partial_cmp(&other.danger) {
            Some(Ordering::Equal) => self.position.partial_cmp(&other.position),
            ord => ord.map(|o| o.reverse()),
        }
    }
}

fn safest_path_danger(grid: &Grid, start: Position, goal: Position) -> Danger {
    let mut explored = HashSet::<Position>::new();
    let mut candidates = BinaryHeap::<Candidate>::new();
    candidates.push(Candidate {
        position: start,
        danger: grid.danger(&start).unwrap(),
    });
    while let Some(Candidate { position, danger }) = candidates.pop() {
        if position == goal {
            return danger;
        }
        for m in MOVES {
            let p = add(position, m);
            if let Some(d) = grid.danger(&p) {
                if explored.insert(p) {
                    candidates.push(Candidate {
                        position: p,
                        danger: danger + d,
                    })
                }
            }
        }
    }

    0
}

fn solve_part2(data: &Data) -> Danger {
    safest_path_danger(data, [0, 0], [14, 14])
}

fn solve_part3(data: &Data) -> i64 {
    let goal = [data.row_count as Idx - 1 as Idx, data.col_count as Idx - 1];
    safest_path_danger(data, [0, 0], goal)
}
