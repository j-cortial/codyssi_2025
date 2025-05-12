use std::ops::Deref;

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

type Id = u64;

#[derive(Clone, Copy, Debug)]
struct Artifact {
    name: &'static str,
    id: Id,
}

type Data = (Vec<Artifact>, [Artifact; 2]);

fn parse_input(input: &'static str) -> Data {
    let sections = input.split_once("\n\n").unwrap();

    let parse = |line: &'static str| {
        let (name, id) = line.split_once(" | ").unwrap();
        Artifact {
            name,
            id: id.parse().unwrap(),
        }
    };

    let tail = {
        let mut lines = sections.1.lines();
        [parse(lines.next().unwrap()), parse(lines.next().unwrap())]
    };

    (sections.0.lines().map(parse).collect(), tail)
}

#[derive(Debug)]
struct TreeNode {
    value: Artifact,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: Artifact) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

fn make_tree(mut artifacts: impl Iterator<Item = Artifact>) -> TreeNode {
    let mut result = TreeNode::new(artifacts.next().unwrap());

    while let Some(artifact) = artifacts.next() {
        let mut node = &mut result;
        loop {
            let subtree = if node.value.id < artifact.id {
                &mut node.left
            } else {
                &mut node.right
            };
            match subtree {
                Some(next_node) => {
                    node = next_node;
                }
                None => {
                    *subtree = Some(Box::new(TreeNode::new(artifact)));
                    break;
                }
            }
        }
    }

    result
}

fn solve_part1(data: &Data) -> Id {
    let tree = make_tree(data.0.iter().copied());

    let mut layer = vec![&tree];
    let mut layer_count = 0;
    let mut max_layer_value = 0;
    while !layer.is_empty() {
        layer_count += 1;
        max_layer_value = max_layer_value.max(layer.iter().map(|&node| node.value.id).sum());
        layer = layer
            .into_iter()
            .flat_map(|node| {
                [&node.right, &node.left]
                    .into_iter()
                    .filter_map(|subtree| subtree.as_ref().map(|node| node.deref()))
            })
            .collect();
    }
    max_layer_value * layer_count
}

fn solve_part2(data: &Data) -> String {
    let tree = make_tree(data.0.iter().copied());

    const ID: Id = 500000;

    let mut node = &tree;
    let mut result = node.value.name.to_owned();

    loop {
        let subtree = if node.value.id < ID {
            &node.left
        } else {
            &node.right
        };
        match subtree {
            Some(next_node) => {
                result += &format!("-{}", next_node.value.name);
                node = next_node;
            }
            None => break,
        }
    }

    result
}

fn solve_part3(data: &Data) -> &'static str {
    let tree = make_tree(data.0.iter().copied());

    let ancestors: Vec<_> = data
        .1
        .iter()
        .map(|artifact| {
            let mut result = vec![tree.value.name];

            let mut node = &tree;
            loop {
                let subtree = if node.value.id < artifact.id {
                    &node.left
                } else {
                    &node.right
                };
                match subtree {
                    Some(next_node) => {
                        result.push(next_node.value.name);
                        node = next_node;
                    }
                    None => break,
                }
            }

            result
        })
        .collect();

    ancestors[0]
        .iter()
        .copied()
        .zip(ancestors[1].iter().copied())
        .take_while(|(a, b)| a == b)
        .last()
        .unwrap()
        .0
}
