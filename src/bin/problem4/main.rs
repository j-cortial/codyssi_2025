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
        .map(|line| line.iter().copied().map(alpha_size).sum::<i64>())
        .sum()
}

fn solve_part2(data: &Data) -> i64 {
    data.iter()
        .map(|line| {
            let length = line.len();
            let to_keep = length / 10;
            let to_remove = length - 2 * to_keep;
            number_size(to_remove)
                + line
                    .iter()
                    .take(to_keep)
                    .chain(line.iter().rev().take(to_keep))
                    .copied()
                    .map(alpha_size)
                    .sum::<i64>()
        })
        .sum()
}

fn solve_part3(data: &Data) -> i64 {
    data.iter()
        .map(|line| {
            let (total, count) =
                line.windows(2)
                    .fold((alpha_size(line[0]), 1), |(total, count), x| {
                        if x[0] == x[1] {
                            (total, count + 1)
                        } else {
                            (total + number_size(count) + alpha_size(x[1]), 1)
                        }
                    });
            total + number_size(count)
        })
        .sum()
}

fn alpha_size(c: u8) -> i64 {
    (c - b'A' + 1) as i64
}

fn number_size(n: usize) -> i64 {
    let mut res = 0;
    let mut remainder = n;
    while remainder > 0 {
        res += remainder % 10;
        remainder /= 10;
    }
    res as i64
}
