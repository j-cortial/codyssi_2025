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

type Location = &'static str;

type Length = i64;

struct Edge {
    begin: Location,
    end: Location,
    length: Length,
}

type Data = Vec<Edge>;

const START: &str = "STT";

fn parse_input(input: &'static str) -> Data {
    input
        .lines()
        .map(|line| {
            let (edge, length) = line.split_once(" | ").unwrap();
            let (begin, end) = edge.split_once(" -> ").unwrap();
            Edge {
                begin,
                end,
                length: length.parse().unwrap(),
            }
        })
        .collect()
}

fn solve_part1(data: &Data) -> Length {
    let mut front = vec![START];
    let mut visited: HashSet<_> = front.iter().copied().collect();
    let mut current_path_length = 0;
    let mut path_lengths: Vec<Length> = vec![current_path_length];

    while !front.is_empty() {
        current_path_length += 1;

        let mut next_front = vec![];
        for &loc in front.iter() {
            for candidate in data
                .iter()
                .filter(|edge| edge.begin == loc)
                .map(|edge| edge.end)
            {
                if visited.insert(candidate) {
                    next_front.push(candidate);
                    path_lengths.push(current_path_length);
                }
            }
        }
        front = next_front;
    }

    path_lengths.into_iter().rev().take(3).product()
}

#[derive(PartialEq, Eq, Ord)]
struct Candidate {
    location: Location,
    distance: Length,
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.distance.partial_cmp(&other.distance) {
            Some(Ordering::Equal) => self.location.partial_cmp(&other.location),
            ord => ord.map(|ord| ord.reverse()),
        }
    }
}

fn solve_part2(data: &Data) -> Length {
    let mut front = BinaryHeap::new();
    front.push(Candidate {
        location: START,
        distance: 0,
    });
    let mut visited = HashSet::new();
    let mut path_lengths = vec![];

    while let Some(Candidate { location, distance }) = front.pop() {
        if visited.insert(location) {
            path_lengths.push(distance);

            for (candidate_location, edge_length) in data
                .iter()
                .filter(|edge| edge.begin == location)
                .map(|edge| (edge.end, edge.length))
            {
                if !visited.contains(candidate_location) {
                    let candidate_distance = distance + edge_length;
                    front.push(Candidate {
                        location: candidate_location,
                        distance: candidate_distance,
                    });
                }
            }
        }
    }

    path_lengths.into_iter().rev().take(3).product()
}

fn solve_part3(data: &Data) -> Length {
    0
}
