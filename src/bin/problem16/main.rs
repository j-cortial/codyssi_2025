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

type Rank = usize;

enum Locus {
    Face,
    Row(Rank),
    Col(Rank),
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

enum Position {
    Front,
    Back,
    Left,
    Right,
    Down,
    Up,
}

type Absorption = u64;

struct Die<const SIZE: usize> {
    faces: [Absorption; 6],
}

impl<const SIZE: usize> Die<SIZE> {
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
    let mut die = Die::<80>::new();

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

fn solve_part2(data: &Data) -> i64 {
    0
}

fn solve_part3(data: &Data) -> i64 {
    0
}
