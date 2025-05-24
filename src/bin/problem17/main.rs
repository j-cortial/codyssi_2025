use std::collections::{BTreeSet, HashMap, HashSet, hash_map};

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

fn steps_to_explore(staircases: &[Staircase]) -> Vec<(StepRank, StaircaseId)> {
    let max_step_rank = staircases[0].end - staircases[0].begin;

    let mut res = vec![];

    let mut active_staircases = BTreeSet::<StaircaseId>::new();

    for step_rank in 0..=max_step_rank {
        let mut next_active_staircases = BTreeSet::new();

        for id in active_staircases.iter().copied().rev() {
            let s = &staircases[id - 1];
            if s.end == step_rank {
                res.push((step_rank, id));
            } else {
                next_active_staircases.insert(id);
            }
        }

        for id in next_active_staircases.iter().copied() {
            res.push((step_rank, id));
        }

        for (id, s) in staircases.iter().enumerate().map(|(id, s)| (id + 1, s)) {
            if s.begin == step_rank {
                res.push((step_rank, id));
                next_active_staircases.insert(id);
            }
        }

        active_staircases = next_active_staircases;
    }

    res
}

fn return_branches(staircases: &[Staircase]) -> Vec<HashMap<StaircaseId, Vec<StaircaseId>>> {
    let max_step_rank = staircases[0].end - staircases[0].begin;

    (0..=max_step_rank)
        .map(|step_rank| {
            let mut branches = HashMap::new();

            for (id, s) in staircases.iter().enumerate().map(|(idx, s)| (idx + 1, s)) {
                if s.end == step_rank {
                    if let Some(return_id) = s.return_staircase {
                        match branches.entry(return_id) {
                            hash_map::Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert_entry(vec![])
                            }
                            hash_map::Entry::Occupied(occupied_entry) => occupied_entry,
                        }
                        .get_mut()
                        .push(id);
                    }
                }
            }

            branches
        })
        .collect()
}

fn solve_part2(data: &Data) -> PathCount {
    let to_explore = steps_to_explore(&data.staircases);
    let branches = return_branches(&data.staircases);

    let mut memory = HashMap::<(StepRank, StaircaseId), PathCount>::new();
    memory.insert((0, 1), 1);

    let max_step_size = *data.allowed_moves.iter().max().unwrap();

    for &(step_rank, staircase_id) in to_explore[1..].into_iter() {
        let mut predecessors = HashSet::new();

        let mut front: HashSet<_> = [(step_rank, staircase_id)].into_iter().collect();

        for step_size in 1..=max_step_size {
            if front.is_empty() {
                break;
            }

            let mut next_front = HashSet::new();

            for (rank, id) in front {
                let staircase = &data.staircases[id - 1];
                if staircase.begin != rank {
                    next_front.insert((rank - 1, id));
                } else if let Some(feeder_id) = staircase.feeding_staircase {
                    next_front.insert((rank, feeder_id));
                }

                for (&candidate, returner_ids) in &branches[rank as usize] {
                    if candidate == id {
                        for &returner_id in returner_ids {
                            next_front.insert((rank, returner_id));
                        }
                    }
                }
            }

            if data.allowed_moves.contains(&step_size) {
                for &step in &next_front {
                    predecessors.insert(step);
                }
            }

            front = next_front;
        }

        let count = predecessors
            .iter()
            .map(|predecessor| memory.get(predecessor).unwrap())
            .sum();
        memory.insert((step_rank, staircase_id), count);
    }

    let max_step_rank = data.staircases[0].end - data.staircases[0].begin;
    *memory.get(&(max_step_rank, 1)).unwrap()
}

fn solve_part3(data: &Data) -> i64 {
    0
}
