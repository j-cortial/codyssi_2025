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

type CoordUnsigned = u8;
type CoordSigned = i8;

type Velocity = [CoordSigned; 4];

type Factor = u8;

struct Rule {
    factors: [Factor; 4],
    divisor: Factor,
    remainder: Factor,
    velocity: Velocity,
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

            let velocity = [
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

const SIZE: [CoordUnsigned; 3] = [10, 15, 60];

#[derive(Clone, Copy)]
enum LastCoord {
    MinusOne,
    Zero,
    PlusOne,
}

impl LastCoord {
    fn value(&self, factor: Factor) -> CoordSigned {
        match self {
            LastCoord::MinusOne => -(factor as CoordSigned),
            LastCoord::Zero => 0,
            LastCoord::PlusOne => factor as CoordSigned,
        }
    }
}

fn solve_part1(data: &Data) -> usize {
    data.iter()
        .map(|rule| {
            (0..SIZE[0])
                .flat_map(|x| (0..SIZE[1]).map(move |y| (x, y)))
                .flat_map(|(x, y)| (0..SIZE[2]).map(move |z| (x, y, z)))
                .flat_map(|v| {
                    [LastCoord::MinusOne, LastCoord::Zero, LastCoord::PlusOne]
                        .map(move |a| ([v.0, v.1, v.2], a))
                })
                .map(|(v, a)| {
                    v.into_iter()
                        .zip(rule.factors[..3].iter().copied())
                        .map(|(v, f)| (v as i64) * (f as i64))
                        .sum::<i64>()
                        + a.value(rule.factors[3]) as i64
                })
                .filter(|&s| {
                    (s + rule.divisor as i64) % (rule.divisor as i64) == (rule.remainder as i64)
                })
                .count()
        })
        .sum()
}

fn solve_part2(data: &Data) -> i64 {
    0
}

fn solve_part3(data: &Data) -> i64 {
    0
}
