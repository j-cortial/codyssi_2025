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

type Data = &'static [u8];

fn parse_input(input: &'static str) -> Data {
    input.as_bytes().trim_ascii_end()
}

fn solve_part1(data: Data) -> i64 {
    data.iter().filter(|b| b.is_ascii_alphabetic()).count() as i64
}

fn solve_part2(data: Data) -> i64 {
    data.iter()
        .filter(|b| b.is_ascii_alphabetic())
        .copied()
        .map(uncorrupted_value)
        .sum()
}

fn solve_part3(data: Data) -> i64 {
    assert!(data[0].is_ascii_alphabetic());
    data.iter()
        .fold((0, 0), |(total, previous), &x| {
            let value = if x.is_ascii_alphabetic() {
                uncorrupted_value(x)
            } else {
                corrupted_value(previous)
            };
            (total + value, value)
        })
        .0
}

fn uncorrupted_value(b: u8) -> i64 {
    (if b.is_ascii_lowercase() {
        b - b'a' + 1
    } else {
        b - b'A' + 27
    }) as i64
}

fn corrupted_value(v: i64) -> i64 {
    ((v as u8 * 2 + (52 - 6)) % 52 + 1) as i64
}
