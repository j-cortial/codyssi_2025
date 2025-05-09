use std::collections::VecDeque;

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

type Amplitude = u64;

const AMPLITUDE_MODULO: Amplitude = 1073741823 + 1;

#[derive(Clone)]
struct Grid {
    row_count: usize,
    col_count: usize,
    data: Vec<Amplitude>,
}

#[derive(Clone, Copy)]
enum Domain {
    All,
    Row(usize),
    Col(usize),
}

#[derive(Clone, Copy)]
enum Operator {
    Shift(usize),
    Add(Amplitude),
    Sub(Amplitude),
    Mul(Amplitude),
}

#[derive(Clone, Copy)]
struct Instruction {
    operator: Operator,
    domain: Domain,
}

#[derive(Clone, Copy)]
enum Control {
    Act,
    Cycle,
}

impl Grid {
    fn apply(&mut self, instruction: &Instruction) {
        match instruction.operator {
            Operator::Shift(n) => match instruction.domain {
                Domain::All => panic!(),
                Domain::Row(i) => {
                    self.shift_row(i, n);
                }
                Domain::Col(j) => {
                    self.shift_col(j, n);
                }
            },
            Operator::Add(y) => {
                let func = |x| (x + y) % AMPLITUDE_MODULO;
                match instruction.domain {
                    Domain::All => self.apply_all(func),
                    Domain::Row(i) => self.apply_row(i, func),
                    Domain::Col(j) => self.apply_col(j, func),
                }
            }
            Operator::Sub(y) => {
                let func = |x| (x + AMPLITUDE_MODULO - y) % AMPLITUDE_MODULO;
                match instruction.domain {
                    Domain::All => self.apply_all(func),
                    Domain::Row(i) => self.apply_row(i, func),
                    Domain::Col(j) => self.apply_col(j, func),
                }
            }
            Operator::Mul(y) => {
                let func = |x| (x * y) % AMPLITUDE_MODULO;
                match instruction.domain {
                    Domain::All => self.apply_all(func),
                    Domain::Row(i) => self.apply_row(i, func),
                    Domain::Col(j) => self.apply_col(j, func),
                }
            }
        }
    }

    fn apply_all<F>(&mut self, func: F)
    where
        F: Fn(Amplitude) -> Amplitude,
    {
        for value in self.data.iter_mut() {
            *value = func(*value);
        }
    }

    fn apply_row<F>(&mut self, i: usize, func: F)
    where
        F: Fn(Amplitude) -> Amplitude,
    {
        for value in self.row_mut(i) {
            *value = func(*value);
        }
    }

    fn apply_col<F>(&mut self, j: usize, func: F)
    where
        F: Fn(Amplitude) -> Amplitude,
    {
        for value in self.col_mut(j) {
            *value = func(*value);
        }
    }

    fn shift_row(&mut self, i: usize, n: usize) {
        let start = i * self.col_count;
        self.data[start..start + self.col_count].rotate_right(n);
    }

    fn shift_col(&mut self, j: usize, n: usize) {
        let mut col: Vec<_> = self.col(j).collect();
        col.rotate_right(n);
        for (dest, src) in self.col_mut(j).zip(col.into_iter()) {
            *dest = src;
        }
    }

    fn row(&self, i: usize) -> impl Iterator<Item = Amplitude> {
        self.data[i * self.col_count..]
            .iter()
            .copied()
            .take(self.col_count)
    }

    fn col(&self, j: usize) -> impl Iterator<Item = Amplitude> {
        self.data[j..]
            .iter()
            .copied()
            .step_by(self.col_count)
            .take(self.row_count)
    }

    fn row_mut(&mut self, i: usize) -> impl Iterator<Item = &mut Amplitude> {
        self.data[i * self.col_count..]
            .iter_mut()
            .take(self.col_count)
    }

    fn col_mut(&mut self, j: usize) -> impl Iterator<Item = &mut Amplitude> {
        self.data[j..]
            .iter_mut()
            .step_by(self.col_count)
            .take(self.row_count)
    }
}

fn highest_amplitude_sum(grid: &Grid) -> Amplitude {
    let highest_row_amplitude: Amplitude = (0..grid.row_count)
        .map(|i| grid.row(i).sum())
        .max()
        .unwrap();
    let highest_col_amplitude = (0..grid.col_count)
        .map(|j| grid.col(j).sum())
        .max()
        .unwrap();
    highest_row_amplitude.max(highest_col_amplitude)
}

struct Data {
    grid: Grid,
    instructions: Vec<Instruction>,
    control: Vec<Control>,
}

fn parse_input(input: &str) -> Data {
    let mut sections = input.split("\n\n");

    let grid = if let Some(section) = sections.next() {
        let row_count = section.lines().count();
        let col_count = section
            .lines()
            .next()
            .map(|line| line.split_ascii_whitespace().count())
            .unwrap();
        let data = section
            .split_ascii_whitespace()
            .map(|token| token.parse().unwrap())
            .collect();
        Grid {
            row_count,
            col_count,
            data,
        }
    } else {
        panic!()
    };

    let instructions = if let Some(section) = sections.next() {
        section
            .lines()
            .map(|line| {
                let tokens: Vec<_> = line.split_ascii_whitespace().collect();

                let operator = match tokens[0] {
                    "SHIFT" => Operator::Shift(tokens[4].parse().unwrap()),
                    "ADD" => Operator::Add(tokens[1].parse().unwrap()),
                    "SUB" => Operator::Sub(tokens[1].parse().unwrap()),
                    "MULTIPLY" => Operator::Mul(tokens[1].parse().unwrap()),
                    _ => panic!(),
                };

                let domain_token = match operator {
                    Operator::Shift(_) => 1,
                    _ => 2,
                };
                let domain = match tokens[domain_token] {
                    "ALL" => Domain::All,
                    "COL" => Domain::Col(tokens[domain_token + 1].parse::<usize>().unwrap() - 1),
                    "ROW" => Domain::Row(tokens[domain_token + 1].parse::<usize>().unwrap() - 1),
                    _ => panic!(),
                };

                Instruction { operator, domain }
            })
            .collect()
    } else {
        panic!()
    };

    let control = if let Some(section) = sections.next() {
        section
            .lines()
            .skip(1)
            .step_by(2)
            .map(|line| match line {
                "ACT" => Control::Act,
                "CYCLE" => Control::Cycle,
                _ => panic!(),
            })
            .collect()
    } else {
        panic!()
    };

    Data {
        grid,
        instructions,
        control,
    }
}

fn solve_part1(data: &Data) -> Amplitude {
    let mut grid = data.grid.clone();

    for instruction in data.instructions.iter() {
        grid.apply(instruction);
    }

    highest_amplitude_sum(&grid)
}

fn solve_part2(data: &Data) -> Amplitude {
    let mut grid = data.grid.clone();

    let mut instructions: VecDeque<_> = data.instructions.iter().copied().collect();

    for &action in data.control.iter() {
        let instruction = instructions.pop_front().unwrap();
        match action {
            Control::Act => grid.apply(&instruction),
            Control::Cycle => instructions.push_back(instruction),
        }
    }

    highest_amplitude_sum(&grid)
}

fn solve_part3(data: &Data) -> i64 {
    0
}
