use std::cmp::Ordering;

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

type Data = Vec<Pos>;

type Pos = [i64; 2];

fn parse_input(input: &'static str) -> Data {
    input
        .lines()
        .map(|line| {
            let tokens = line.trim_matches(['(', ')']).split_once(", ").unwrap();
            [tokens.0.parse().unwrap(), tokens.1.parse().unwrap()]
        })
        .collect()
}

fn solve_part1(data: &Data) -> i64 {
    let minmax_distances = data.iter().fold((i64::MAX, 0), |acc, x| {
        let d = distance(x, &Pos::default());
        (acc.0.min(d), acc.1.max(d))
    });
    minmax_distances.1 - minmax_distances.0
}

fn solve_part2(data: &Data) -> i64 {
    let comparator = Comparator::new(Pos::default());
    let closest_island = data.iter().min_by(|a, b| comparator.apply(a, b)).unwrap();
    data.iter()
        .filter_map(|pos| {
            if pos == closest_island {
                None
            } else {
                Some(distance(closest_island, pos))
            }
        })
        .min()
        .unwrap()
}

fn solve_part3(data: &Data) -> i64 {
    let mut remainder = data.clone();
    let mut explored = vec![Pos::default()];

    while !remainder.is_empty() {
        let comparator = Comparator::new(*explored.last().unwrap());
        let index = remainder
            .iter()
            .enumerate()
            .min_by(|a, b| comparator.apply(a.1, b.1))
            .unwrap()
            .0;
        explored.push(remainder.swap_remove(index));
    }

    explored
        .windows(2)
        .map(|leg| distance(&leg[0], &leg[1]))
        .sum()
}

fn distance(a: &Pos, b: &Pos) -> i64 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

struct Comparator {
    reference: Pos,
}

impl Comparator {
    fn new(reference: Pos) -> Self {
        Self { reference }
    }

    fn apply(&self, a: &Pos, b: &Pos) -> Ordering {
        distance(a, &self.reference)
            .cmp(&distance(b, &self.reference))
            .then(a[0].cmp(&b[0]))
            .then(a[1].cmp(&b[1]))
    }
}
