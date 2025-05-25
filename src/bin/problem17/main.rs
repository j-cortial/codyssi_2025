use std::{
    collections::{BTreeSet, HashMap, HashSet, hash_map},
    fmt::Display,
};

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Node {
    staircase_id: StaircaseId,
    step_rank: StepRank,
}

impl Node {
    fn new(staircase_id: StaircaseId, step_rank: StepRank) -> Self {
        Self {
            staircase_id,
            step_rank,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}_{}", self.staircase_id, self.step_rank)
    }
}

fn nodes_to_explore(staircases: &[Staircase]) -> Vec<Node> {
    let max_step_rank = staircases[0].end - staircases[0].begin;

    let mut res = vec![];

    let mut active_staircases = BTreeSet::<StaircaseId>::new();

    for step_rank in 0..=max_step_rank {
        let mut next_active_staircases = BTreeSet::new();

        for id in active_staircases.iter().copied().rev() {
            let s = &staircases[id - 1];
            if s.end == step_rank {
                res.push(Node::new(id, step_rank));
            } else {
                next_active_staircases.insert(id);
            }
        }

        for id in next_active_staircases.iter().copied() {
            res.push(Node::new(id, step_rank));
        }

        for (id, s) in staircases.iter().enumerate().map(|(id, s)| (id + 1, s)) {
            if s.begin == step_rank {
                res.push(Node::new(id, step_rank));
                next_active_staircases.insert(id);
            }
        }

        active_staircases = next_active_staircases;
    }

    res
}

fn feeding_branches(staircases: &[Staircase]) -> Vec<HashMap<StaircaseId, Vec<StaircaseId>>> {
    let max_step_rank = staircases[0].end - staircases[0].begin;

    (0..=max_step_rank)
        .map(|step_rank| {
            let mut branches = HashMap::new();

            for (id, s) in staircases.iter().enumerate().map(|(idx, s)| (idx + 1, s)) {
                if s.begin == step_rank {
                    if let Some(return_id) = s.feeding_staircase {
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

fn successors(staircases: &[Staircase], allowed_moves: &[StepCount]) -> HashMap<Node, Vec<Node>> {
    let branches = feeding_branches(staircases);
    let max_step_size = *allowed_moves.iter().max().unwrap();

    nodes_to_explore(staircases)
        .into_iter()
        .rev()
        .map(|node| {
            let mut res = BTreeSet::new();

            let mut front: HashSet<_> = [node].into_iter().collect();

            for step_size in 1..=max_step_size {
                if front.is_empty() {
                    break;
                }

                let mut next_front = HashSet::new();

                for node in front {
                    let staircase = &staircases[node.staircase_id - 1];
                    if staircase.end != node.step_rank {
                        next_front.insert(Node::new(node.staircase_id, node.step_rank + 1));
                    } else if let Some(return_id) = staircase.return_staircase {
                        next_front.insert(Node::new(return_id, node.step_rank));
                    }

                    for (&candidate, feeder_ids) in &branches[node.step_rank as usize] {
                        if candidate == node.staircase_id {
                            for &returner_id in feeder_ids {
                                next_front.insert(Node::new(returner_id, node.step_rank));
                            }
                        }
                    }
                }

                if allowed_moves.contains(&step_size) {
                    for &step in &next_front {
                        res.insert(step);
                    }
                }

                front = next_front;
            }
            (node, res.into_iter().collect())
        })
        .collect()
}

fn allowed_starting_paths(
    staircases: &[Staircase],
    allowed_moves: &[StepCount],
) -> HashMap<Node, PathCount> {
    let successors = successors(staircases, allowed_moves);

    let mut res = HashMap::new();

    let end_node = Node::new(1, staircases[0].end);

    res.insert(end_node, 1);

    for node in nodes_to_explore(staircases).into_iter().rev().skip(1) {
        let count = successors
            .get(&node)
            .unwrap()
            .iter()
            .map(|predecessor| res.get(predecessor).unwrap())
            .sum();
        res.insert(node, count);
    }

    res
}

fn solve_part2(data: &Data) -> PathCount {
    let begin_node = Node::new(1, data.staircases[0].begin);

    *allowed_starting_paths(&data.staircases, &data.allowed_moves)
        .get(&begin_node)
        .unwrap()
}

fn solve_part3(data: &Data) -> String {
    const TARGET_PATH_RANK: PathCount = 100000000000000000000000000000;

    let successors = successors(&data.staircases, &data.allowed_moves);
    let allowed_paths = allowed_starting_paths(&data.staircases, &data.allowed_moves);

    let begin_node = Node::new(1, data.staircases[0].begin);
    let end_node = Node::new(1, data.staircases[0].end);

    let target_path_rank = TARGET_PATH_RANK.min(*allowed_paths.get(&begin_node).unwrap());

    let mut path = vec![begin_node];
    let mut dominated_path_count: PathCount = 0;

    let mut node = *path.last().unwrap();
    while node != end_node {
        let (next_node, next_dominated_path_count) = successors[&node]
            .iter()
            .zip(
                successors[&node]
                    .iter()
                    .scan(dominated_path_count, |acc, x| {
                        let res = Some(*acc);
                        *acc += *allowed_paths.get(x).unwrap();
                        res
                    }),
            )
            .take_while(|(_, n)| *n < target_path_rank)
            .last()
            .unwrap();

        dominated_path_count = next_dominated_path_count;
        path.push(*next_node);
        node = *path.last().unwrap();
    }

    format!(
        "{}",
        path.into_iter()
            .map(|node| format!("{}", node))
            .collect::<Vec<_>>()
            .join("-")
    )
}
