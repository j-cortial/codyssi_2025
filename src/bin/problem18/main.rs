use std::{collections::BTreeSet, iter::once};

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

type Coord = u8;
type SignedCoord = i8;

type Position = [Coord; 4];
type Velocity = [SignedCoord; 4];

const SIZE: [Coord; 4] = [10, 15, 60, 3];
const OFFSET: [Coord; 4] = [0, 0, 0, 1];

type Factor = u8;

type Time = u16;

struct Rule {
    factors: [Factor; 4],
    divisor: Factor,
    remainder: Factor,
    velocity: Velocity,
}

impl Rule {
    fn holds(&self, time: Time, position: &Position) -> bool {
        let sum = position
            .into_iter()
            .zip(OFFSET.iter().zip(SIZE.iter()))
            .zip(self.velocity.iter())
            .map(|((&p, (&o, &s)), &v)| {
                let p0 = p as i64 - (time as i64 * v as i64);
                let p = if p0 >= 0 {
                    p0 as u64 % s as u64
                } else {
                    let n = (-p0 + s as i64) as i64 / s as i64;
                    (p0 + (n * s as i64)) as u64 % s as u64
                };
                (p as Coord, o)
            })
            .zip(self.factors.iter())
            .map(|((p, o), &f)| (p as i64 - o as i64) * (f as i64))
            .sum::<i64>();
        (sum + self.divisor as i64) as u64 % (self.divisor as u64) == (self.remainder as u64)
    }

    fn holds_for_initial_time(&self, position: &Position) -> bool {
        self.holds(0, position)
    }
}

fn position_is_safe(position: &Position, time: Time, rules: &[Rule]) -> bool {
    position == &OFFSET || !rules.iter().any(|rule| rule.holds(time, position))
}

fn safe_successors(position: &Position, time: Time, rules: &[Rule]) -> Vec<Position> {
    (0..3)
        .flat_map(|i| {
            [
                if position[i] > 0 { Some(-1) } else { None },
                if position[i] + 1 < SIZE[i] {
                    Some(1)
                } else {
                    None
                },
            ]
            .into_iter()
            .filter_map(move |d| {
                d.map(|d| {
                    let mut res = *position;
                    res[i] = (res[i] as SignedCoord + d) as Coord;
                    res
                })
            })
        })
        .chain(once(position).copied())
        .filter(|candidate| position_is_safe(candidate, time + 1, &rules))
        .collect()
}

type Data = Vec<Rule>;

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let (_, content) = line.split_once(": ").unwrap();
            let (condition, velocity) = content.split_once(" | ").unwrap();

            let condition_tokens: Vec<_> = condition.split_ascii_whitespace().collect();

            let mut factor_iterator = condition_tokens[0]
                .split('+')
                .map(|token| &token[..token.len() - 1]);
            let factors = [
                factor_iterator.next().unwrap().parse().unwrap(),
                factor_iterator.next().unwrap().parse().unwrap(),
                factor_iterator.next().unwrap().parse().unwrap(),
                factor_iterator.next().unwrap().parse().unwrap(),
            ];

            let divisor = condition_tokens[2].parse().unwrap();
            let remainder = condition_tokens[5].parse().unwrap();

            let mut coordinate_iterator = velocity[16..]
                .trim_matches(|c| c == '(' || c == ')')
                .split(", ");

            let velocity: [SignedCoord; 4] = [
                coordinate_iterator.next().unwrap().parse().unwrap(),
                coordinate_iterator.next().unwrap().parse().unwrap(),
                coordinate_iterator.next().unwrap().parse().unwrap(),
                coordinate_iterator.next().unwrap().parse().unwrap(),
            ];

            Rule {
                factors,
                divisor,
                remainder,
                velocity,
            }
        })
        .collect()
}

fn solve_part1(data: &Data) -> usize {
    data.into_iter()
        .map(|rule| {
            (0..SIZE[0])
                .flat_map(|x| (0..SIZE[1]).map(move |y| (x, y)))
                .flat_map(|(x, y)| (0..SIZE[2]).map(move |z| (x, y, z)))
                .flat_map(|p| (0..SIZE[3]).map(move |a| ([p.0, p.1, p.2, a])))
                .filter(|p| rule.holds_for_initial_time(p))
                .count()
        })
        .sum()
}

fn solve_part2(data: &Data) -> Time {
    const EXIT: Position = [9 + OFFSET[0], 14 + OFFSET[1], 59 + OFFSET[2], 0 + OFFSET[3]];
    let mut front: BTreeSet<_> = once(OFFSET).collect();

    for time in 0.. {
        if front.contains(&EXIT) {
            return time;
        }

        front = front
            .into_iter()
            .flat_map(|position| safe_successors(&position, time, &data).into_iter())
            .collect();
    }

    unreachable!()
}

fn solve_part3(data: &Data) -> i64 {
    0
}
