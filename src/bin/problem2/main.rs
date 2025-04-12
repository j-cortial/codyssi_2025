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

type Int = i64;

enum Op {
    Plus,
    Mult,
    Pow,
}

struct Func {
    op: Op,
    value: Int,
}

impl Func {
    fn apply(&self, i: Int) -> Int {
        match self.op {
            Op::Plus => i + self.value,
            Op::Mult => i * self.value,
            Op::Pow => i.pow(self.value as u32),
        }
    }
}

type Data = (Vec<Func>, Vec<Int>);

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();

    let mut funcs = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let tokens: Vec<_> = line.split_whitespace().collect();
        let op = match tokens[2] {
            "ADD" => Op::Plus,
            "MULTIPLY" => Op::Mult,
            "RAISE" => Op::Pow,
            _ => panic!(),
        };
        let value = tokens.last().unwrap().parse().unwrap();
        funcs.push(Func { op, value });
    }

    let values = lines.map(|line| line.parse().unwrap()).collect();

    (funcs, values)
}

fn solve_part1(data: &Data) -> Int {
    let mut prices: Vec<_> = data.1.iter().copied().collect();
    prices.sort();
    let median_price = prices[prices.len() / 2];
    apply(median_price, &data.0)
}

fn solve_part2(data: &Data) -> Int {
    let even_prices = data.1.iter().copied().filter(|i| i % 2 == 0).sum();
    apply(even_prices, &data.0)
}

fn solve_part3(data: &Data) -> Int {
    let mut data: Vec<_> = data
        .1
        .iter()
        .copied()
        .map(|i| (i, apply(i, &data.0)))
        .collect();
    data.sort_unstable_by_key(|item| item.1);
    const UPPER_BOUND: Int = 15000000000000;
    data.iter()
        .rev()
        .find(|&(_, i)| *i <= UPPER_BOUND)
        .map(|&(i, _)| i)
        .unwrap()
}

fn apply(price: Int, funcs: &[Func]) -> Int {
    funcs.iter().rev().fold(price, |acc, f| f.apply(acc))
}
