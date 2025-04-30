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

struct Data {
    current_frequencies: Vec<i64>,
    swap_instructions: Vec<(usize, usize)>,
    test_index: usize,
}

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();

    let mut current_frequencies = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        current_frequencies.push(line.parse().unwrap());
    }

    let mut swap_instructions = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let tokens = line.split_once('-').unwrap();
        swap_instructions.push((tokens.0.parse().unwrap(), tokens.1.parse().unwrap()));
    }

    let test_index = lines.next().unwrap().parse().unwrap();

    Data {
        current_frequencies,
        swap_instructions,
        test_index,
    }
}

fn solve_part1(data: &Data) -> i64 {
    let mut frequencies = data.current_frequencies.clone();
    for &swap in data.swap_instructions.iter() {
        perform_swap(&mut frequencies, swap);
    }
    frequencies[data.test_index - 1]
}

fn solve_part2(data: &Data) -> i64 {
    let mut frequencies = data.current_frequencies.clone();
    for swaps in data.swap_instructions.windows(2) {
        perform_swap(&mut frequencies, swaps[0]);
        perform_swap(&mut frequencies, (swaps[0].0, swaps[1].0));
    }
    perform_swap(&mut frequencies, *data.swap_instructions.last().unwrap());
    perform_swap(
        &mut frequencies,
        (
            data.swap_instructions.last().unwrap().0,
            data.swap_instructions[0].0,
        ),
    );
    frequencies[data.test_index - 1]
}

fn solve_part3(data: &Data) -> i64 {
    let mut frequencies = data.current_frequencies.clone();
    for swap in data.swap_instructions.iter().copied() {
        let start = swap.0.min(swap.1) - 1;
        let mid = swap.0.max(swap.1) - 1;
        let block_length = (frequencies.len() - mid).min(mid - start);
        let (head, tail) = frequencies[start..].split_at_mut(mid - start);
        head[..block_length].swap_with_slice(&mut tail[..block_length]);
    }
    frequencies[data.test_index - 1]
}

fn perform_swap(frequencies: &mut [i64], swap: (usize, usize)) {
    frequencies.swap(swap.0 - 1, swap.1 - 1);
}
