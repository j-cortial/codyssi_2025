use std::array::from_fn;

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

type Value = u8;

type Index = usize;

#[derive(Clone, Copy)]
enum Locus {
    Face,
    Row(Index),
    Col(Index),
}

struct Instruction {
    locus: Locus,
    value: Value,
}

enum Twist {
    Left,
    Right,
    Down,
    Up,
}

struct Data {
    instructions: Vec<Instruction>,
    twists: Vec<Twist>,
}

fn parse_input(input: &str) -> Data {
    let sections = input.split_once("\n\n").unwrap();

    let instructions = sections
        .0
        .lines()
        .map(|line| {
            let components = line.split_once(" - VALUE ").unwrap();
            let locus = {
                let mut tokens = components.0.split_ascii_whitespace();
                match tokens.next().unwrap() {
                    "FACE" => Locus::Face,
                    "ROW" => Locus::Row(tokens.next().unwrap().parse().unwrap()),
                    "COL" => Locus::Col(tokens.next().unwrap().parse().unwrap()),
                    _ => panic!(),
                }
            };
            let value = components.1.parse().unwrap();
            Instruction { locus, value }
        })
        .collect();

    let twists = sections
        .1
        .trim()
        .bytes()
        .map(|c| match c {
            b'L' => Twist::Left,
            b'R' => Twist::Right,
            b'D' => Twist::Down,
            b'U' => Twist::Up,
            _ => panic!(),
        })
        .collect();

    Data {
        instructions,
        twists,
    }
}

#[derive(Clone, Copy)]
enum Position {
    Front,
    Back,
    Left,
    Right,
    Down,
    Up,
}

type Absorption = u64;

struct SimpleDie<const SIZE: usize> {
    faces: [Absorption; 6],
}

impl<const SIZE: usize> SimpleDie<SIZE> {
    fn new() -> Self {
        Self {
            faces: Default::default(),
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        self.faces[Position::Front as usize] += match instruction.locus {
            Locus::Face => SIZE * SIZE,
            Locus::Row(_) => SIZE,
            Locus::Col(_) => SIZE,
        } as Absorption
            * instruction.value as Absorption;
    }

    fn rotate(&mut self, twist: &Twist) {
        match twist {
            Twist::Left => {
                self.swap(Position::Front, Position::Left);
                self.swap(Position::Left, Position::Back);
                self.swap(Position::Back, Position::Right);
            }
            Twist::Right => {
                self.swap(Position::Front, Position::Right);
                self.swap(Position::Right, Position::Back);
                self.swap(Position::Back, Position::Left);
            }
            Twist::Down => {
                self.swap(Position::Front, Position::Down);
                self.swap(Position::Down, Position::Back);
                self.swap(Position::Back, Position::Up);
            }
            Twist::Up => {
                self.swap(Position::Front, Position::Up);
                self.swap(Position::Up, Position::Back);
                self.swap(Position::Back, Position::Down);
            }
        }
    }

    fn swap(&mut self, a: Position, b: Position) {
        self.faces.swap(a as usize, b as usize);
    }
}

fn solve_part1(data: &Data) -> Absorption {
    let mut die = SimpleDie::<80>::new();

    let mut instructions = data.instructions.iter();
    die.apply(instructions.next().unwrap());

    for (instruction, twist) in instructions.zip(data.twists.iter()) {
        die.rotate(twist);
        die.apply(instruction);
    }

    let mut absorptions: Vec<_> = die.faces.into_iter().collect();
    absorptions.sort_unstable();
    absorptions.into_iter().rev().take(2).product()
}

#[derive(Clone, Copy)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn turn_upside_down(&mut self) {
        *self = match self {
            Orientation::Up => Orientation::Down,
            Orientation::Right => Orientation::Left,
            Orientation::Down => Orientation::Up,
            Orientation::Left => Orientation::Right,
        };
    }

    fn turn_clockwise(&mut self) {
        *self = match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }

    fn turn_counterclockwise(&mut self) {
        *self = match self {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        };
    }
}

struct Action<const SIZE: usize> {
    locus: Locus,
    value: Value,
}

impl<const SIZE: usize> Action<SIZE> {
    fn new(instruction: &Instruction, orientation: Orientation) -> Self {
        let locus = match orientation {
            Orientation::Up => instruction.locus,
            Orientation::Right => match instruction.locus {
                Locus::Face => Locus::Face,
                Locus::Row(index) => Locus::Col(index),
                Locus::Col(index) => Locus::Row(SIZE + 1 - index),
            },
            Orientation::Down => match instruction.locus {
                Locus::Face => Locus::Face,
                Locus::Row(index) => Locus::Row(SIZE + 1 - index),
                Locus::Col(index) => Locus::Col(SIZE + 1 - index),
            },
            Orientation::Left => match instruction.locus {
                Locus::Face => Locus::Face,
                Locus::Row(index) => Locus::Col(SIZE + 1 - index),
                Locus::Col(index) => Locus::Row(index),
            },
        };
        Self {
            locus,
            value: instruction.value,
        }
    }
}

struct Grid<const SIZE: usize> {
    values: [[Value; SIZE]; SIZE],
}

impl<const SIZE: usize> Grid<SIZE> {
    fn new() -> Self {
        Self {
            values: [[1; SIZE]; SIZE],
        }
    }

    fn apply(&mut self, action: &Action<SIZE>) {
        match action.locus {
            Locus::Face => {
                for row in &mut self.values {
                    for entry in row {
                        *entry = add(*entry, action.value);
                    }
                }
            }
            Locus::Row(index) => {
                for entry in &mut self.values[index - 1] {
                    *entry = add(*entry, action.value);
                }
            }
            Locus::Col(index) => {
                for entry in self.values.iter_mut().map(|row| &mut row[index - 1]) {
                    *entry = add(*entry, action.value);
                }
            }
        }
    }

    fn dominant_sum(&self) -> u64 {
        let dominant_row_sum: u64 = self
            .values
            .iter()
            .map(|row| row.iter().map(|&v| v as u64).sum())
            .max()
            .unwrap();
        let dominant_col_sum = (0..SIZE)
            .map(|j| self.values.iter().map(|row| row[j] as u64).sum())
            .max()
            .unwrap();
        dominant_row_sum.max(dominant_col_sum)
    }
}

fn add(a: Value, b: Value) -> Value {
    ((a + b - 1) % 100) + 1
}

struct Die<const SIZE: usize> {
    values: [Grid<SIZE>; 6],
    orientations: [Orientation; 6],
}

impl<const SIZE: usize> Die<SIZE> {
    fn new() -> Self {
        Self {
            values: from_fn(|_| Grid::new()),
            orientations: [Orientation::Up; 6],
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        let action = Action::new(instruction, self.orientations[Position::Front as usize]);
        let grid = &mut self.values[Position::Front as usize];
        grid.apply(&action);
    }

    fn rotate(&mut self, twist: &Twist) {
        match twist {
            Twist::Left => {
                self.orientations[Position::Back as usize].turn_upside_down();
                self.swap(Position::Front, Position::Left);
                self.swap(Position::Left, Position::Back);
                self.swap(Position::Back, Position::Right);
                self.orientations[Position::Down as usize].turn_clockwise();
                self.orientations[Position::Up as usize].turn_counterclockwise();
                self.orientations[Position::Back as usize].turn_upside_down();
            }
            Twist::Right => {
                self.orientations[Position::Back as usize].turn_upside_down();
                self.swap(Position::Front, Position::Right);
                self.swap(Position::Right, Position::Back);
                self.swap(Position::Back, Position::Left);
                self.orientations[Position::Down as usize].turn_counterclockwise();
                self.orientations[Position::Up as usize].turn_clockwise();
                self.orientations[Position::Back as usize].turn_upside_down();
            }
            Twist::Down => {
                self.swap(Position::Front, Position::Down);
                self.swap(Position::Down, Position::Back);
                self.swap(Position::Back, Position::Up);
                self.orientations[Position::Left as usize].turn_counterclockwise();
                self.orientations[Position::Right as usize].turn_clockwise();
            }
            Twist::Up => {
                self.swap(Position::Front, Position::Up);
                self.swap(Position::Up, Position::Back);
                self.swap(Position::Back, Position::Down);
                self.orientations[Position::Left as usize].turn_clockwise();
                self.orientations[Position::Right as usize].turn_counterclockwise();
            }
        }
    }

    fn swap(&mut self, a: Position, b: Position) {
        self.values.swap(a as usize, b as usize);
        self.orientations.swap(a as usize, b as usize);
    }
}

fn solve_part2(data: &Data) -> u128 {
    let mut die = Die::<80>::new();

    let mut instructions = data.instructions.iter();
    die.apply(instructions.next().unwrap());

    for (instruction, twist) in instructions.zip(data.twists.iter()) {
        die.rotate(twist);
        die.apply(instruction);
    }

    die.values
        .iter()
        .map(|face| face.dominant_sum() as u128)
        .product()
}

fn solve_part3(data: &Data) -> i64 {
    0
}
