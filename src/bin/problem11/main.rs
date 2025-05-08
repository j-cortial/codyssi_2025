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

struct Number {
    representation: &'static [u8],
    base: u8,
}

impl Number {
    fn value(&self) -> u64 {
        self.representation
            .iter()
            .copied()
            .fold(0, |acc, x| acc * self.base as u64 + value(x) as u64)
    }
}

fn digits(number: u64, base: u8) -> Vec<u8> {
    if number == 0 {
        return vec![b'0'];
    }
    let base = base as u64;
    let mut remainder = number;
    let mut result = vec![];
    while remainder > 0 {
        result.push((remainder % base) as u8);
        remainder /= base;
    }
    result.reverse();
    result
}

const ADDITIONAL_CHARS: &[u8] = b"!@#$%^";

fn value(c: u8) -> u8 {
    if c.is_ascii_digit() {
        c - b'0'
    } else if c.is_ascii_uppercase() {
        c - b'A' + 10
    } else if c.is_ascii_lowercase() {
        c - b'a' + 36
    } else if let Some(i) = ADDITIONAL_CHARS.iter().copied().position(|a| a == c) {
        62 + i as u8
    } else {
        panic!();
    }
}

fn as_char(digit: u8) -> u8 {
    if digit <= 9 {
        b'0' + digit
    } else if digit <= 35 {
        b'A' + digit - 10
    } else if digit <= 61 {
        b'a' + digit - 36
    } else if digit <= 67 {
        ADDITIONAL_CHARS[digit as usize - 62]
    } else {
        panic!();
    }
}

type Data = Vec<Number>;

fn parse_input(input: &'static str) -> Data {
    input
        .lines()
        .map(|line| {
            let (representation, base) = line.split_once(' ').unwrap();
            Number {
                representation: representation.as_bytes(),
                base: base.parse().unwrap(),
            }
        })
        .collect()
}

fn solve_part1(data: &Data) -> u64 {
    data.iter().map(|number| number.value()).max().unwrap()
}

fn solve_part2(data: &Data) -> String {
    let digits = digits(data.iter().map(|number| number.value()).sum(), 68);
    let chars: Vec<_> = digits.into_iter().map(as_char).collect();
    str::from_utf8(&chars).unwrap().to_owned()
}

fn solve_part3(data: &Data) -> u64 {
    let sum: u64 = data.iter().map(|number| number.value()).sum();
    for base in 2.. {
        let square = base * base;
        let max_value = square * square - 1;
        if max_value >= sum {
            return base;
        }
    }
    0
}
