use std::ops::RangeInclusive;

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

type Int = i16;

type Data = Vec<[RangeInclusive<Int>; 2]>;

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let pair = line.split_once(' ').unwrap();
            let parse_range = |token: &str| {
                let bounds = token.split_once('-').unwrap();
                bounds.0.parse().unwrap()..=bounds.1.parse().unwrap()
            };
            [parse_range(pair.0), parse_range(pair.1)]
        })
        .collect()
}

fn solve_part1(data: &Data) -> usize {
    data.iter()
        .flat_map(|pair| pair.iter().map(|rng| rng.len()))
        .sum()
}

fn solve_part2(data: &Data) -> usize {
    data.iter()
        .map(|pair| {
            (*pair[0].start().min(pair[1].start())..=*pair[0].end().max(pair[1].end())).len()
                - (*pair[0].end().min(pair[1].end()) + 1..*pair[0].start().max(pair[1].start()))
                    .len()
        })
        .sum()
}

fn solve_part3(data: &Data) -> usize {
    data.windows(2)
        .map(|w| {
            let mut ranges: Vec<_> = w.iter().flat_map(|p| p.iter().cloned()).collect();
            ranges.sort_unstable_by_key(|rng| *rng.start());
            ranges[1..]
                .iter()
                .fold((ranges[0].len(), *ranges[0].end()), |(len, end), rng| {
                    (
                        len + ((end + 1).max(*rng.start())..=*rng.end()).len(),
                        end.max(*rng.end()),
                    )
                })
                .0
        })
        .max()
        .unwrap()
}
