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

type StaircaseId = usize;

type StepRank = u8;

type StepCount = u8;

type PathCount = u128;

struct Staircase {
    begin: StepRank,
    end: StepRank,
    feeding_staircase: Option<StaircaseId>,
    return_staircase: Option<StaircaseId>,
}

struct Data {
    staircases: Vec<Staircase>,
    allowed_moves: Vec<StepCount>,
}

fn parse_input(input: &str) -> Data {
    let sections = input.split_once("\n\n").unwrap();

    let staircases = sections
        .0
        .lines()
        .map(|line| {
            let tokens: Vec<_> = line.split_ascii_whitespace().collect();

            let begin = tokens[2].parse().unwrap();
            let end = tokens[4].parse().unwrap();
            let feeding_staircase = match tokens[7] {
                "START" => None,
                token => Some(token[1..].parse().unwrap()),
            };
            let return_staircase = match tokens[9] {
                "END" => None,
                token => Some(token[1..].parse().unwrap()),
            };

            Staircase {
                begin,
                end,
                feeding_staircase,
                return_staircase,
            }
        })
        .collect();

    let allowed_moves = sections
        .1
        .trim_ascii()
        .split_once(" : ")
        .unwrap()
        .1
        .split(", ")
        .map(|token| token.parse().unwrap())
        .collect();

    Data {
        staircases,
        allowed_moves,
    }
}

fn path_count(step_count: StepCount, allowed_moves: &[StepCount]) -> PathCount {
    let mut memory = vec![vec![0; allowed_moves.len() + 1]; step_count as usize + 1];
    memory[0].fill(1);

    for (i, c) in (1..=step_count).map(|c| (c as usize, c)) {
        for (j, m) in allowed_moves
            .iter()
            .copied()
            .enumerate()
            .map(|(j, m)| (j + 1, m))
        {
            memory[i][j] = memory[i][j - 1]
                + if m > c {
                    0
                } else {
                    let k = (c - m) as usize;
                    (0..=k).map(|n| memory[n][j - 1] * memory[k - n][j]).sum()
                };
        }
    }

    *memory.last().unwrap().last().unwrap()
}

fn solve_part1(data: &Data) -> PathCount {
    path_count(
        data.staircases[0].end - data.staircases[0].begin,
        &data.allowed_moves,
    )
}

fn solve_part2(data: &Data) -> i64 {
    0
}

fn solve_part3(data: &Data) -> i64 {
    0
}
