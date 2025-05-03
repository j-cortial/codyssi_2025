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

type Data = Vec<&'static [u8]>;

fn parse_input(input: &'static str) -> Data {
    input.lines().map(str::as_bytes).collect()
}

fn solve_part1(data: &Data) -> i64 {
    data.iter()
        .map(|&line| line.iter().copied().filter(u8::is_ascii_alphabetic).count())
        .sum::<usize>() as i64
}

fn solve_part2(data: &Data) -> i64 {
    data.iter()
        .map(|&line| {
            line.iter()
                .fold(0i64, |acc, &c| {
                    if c.is_ascii_alphabetic() || c == b'-' {
                        acc + 1
                    } else {
                        assert!(c.is_ascii_digit());
                        acc - 1
                    }
                })
                .abs()
        })
        .sum::<i64>()
}

fn solve_part3(data: &Data) -> i64 {
    data.iter()
        .map(|&line| {
            let (total, running) = line.iter().fold((0i64, 0i64), |(total, running), &c| {
                if c == b'-' {
                    (total + running.abs() + 1, 0)
                } else if c.is_ascii_alphabetic() {
                    (total, running + 1)
                } else {
                    assert!(c.is_ascii_digit());
                    (total, running - 1)
                }
            });
            total + running.abs()
        })
        .sum::<i64>()
}
