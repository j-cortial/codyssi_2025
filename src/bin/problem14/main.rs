use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!(
        "The answer to part 1 is {answer1}",
        answer1 = solve_part1(&input)
    );
    println!(
        "The answer to part 1 is {answer2}",
        answer2 = solve_part2(&input)
    );
    println!(
        "The answer to part 3 is {answer3}",
        answer3 = solve_part3(&input)
    );
}

#[derive(Clone, Copy)]
struct Item {
    quality: i64,
    cost: i64,
    material: i64,
}

type Data = Vec<Item>;

fn parse_input(input: &'static str) -> Data {
    input
        .lines()
        .map(|line| {
            let tokens: Vec<_> = line.split_ascii_whitespace().collect();
            Item {
                quality: tokens[5].trim_end_matches(',').parse().unwrap(),
                cost: tokens[8].trim_end_matches(',').parse().unwrap(),
                material: tokens[12].parse().unwrap(),
            }
        })
        .collect()
}

fn rank(a: &Item, b: &Item) -> Ordering {
    match a.quality.cmp(&b.quality) {
        Ordering::Equal => a.cost.cmp(&b.cost),
        otherwise => otherwise,
    }
}

fn solve_part1(data: &Data) -> i64 {
    let mut items = data.clone();
    items.sort_by(rank);
    items
        .into_iter()
        .rev()
        .take(5)
        .map(|item| item.material)
        .sum()
}

type Scenario = (usize, i64);

fn optimal_quality(scenario: Scenario, items: &[Item], memory: &mut HashMap<Scenario, i64>) -> i64 {
    match memory.get(&scenario) {
        Some(res) => *res,
        None => {
            let res = optimal_quality_impl(scenario, items, memory);
            memory.insert(scenario, res);
            res
        }
    }
}

fn optimal_quality_impl(
    (item_count, cost_upper_bound): Scenario,
    items: &[Item],
    memory: &mut HashMap<Scenario, i64>,
) -> i64 {
    if item_count == 0 || cost_upper_bound == 0 {
        return 0;
    }

    let prev = optimal_quality((item_count - 1, cost_upper_bound), items, memory);

    let item_cost = items[item_count - 1].cost;
    if item_cost > cost_upper_bound {
        prev
    } else {
        let other = optimal_quality(
            (item_count - 1, cost_upper_bound - item_cost),
            items,
            memory,
        );

        prev.max(other + items[item_count - 1].quality)
    }
}

fn optimal_set(
    (item_count, cost_upper_bound): Scenario,
    items: &[Item],
    memory: &HashMap<Scenario, i64>,
) -> Vec<usize> {
    if item_count == 0 {
        return vec![];
    }

    if memory
        .get(&(item_count, cost_upper_bound))
        .copied()
        .unwrap()
        > memory
            .get(&(item_count - 1, cost_upper_bound))
            .copied()
            .unwrap_or_default()
    {
        let mut res = optimal_set(
            (
                item_count - 1,
                cost_upper_bound - items[item_count - 1].cost,
            ),
            items,
            memory,
        );
        res.push(item_count - 1);
        res
    } else {
        optimal_set((item_count - 1, cost_upper_bound), items, memory)
    }
}

fn optimal_synthesis(cost_upper_bound: i64, items: &[Item]) -> i64 {
    let scenario = (items.len(), cost_upper_bound);

    let mut memory = HashMap::new();
    let optimal_quality = optimal_quality(scenario, items, &mut memory);
    let optimal_set = optimal_set(scenario, items, &memory);

    let optimal_material_quantity = optimal_set
        .into_iter()
        .map(|i| items[i].material)
        .sum::<i64>();
    optimal_material_quantity * optimal_quality
}

fn solve_part2(data: &Data) -> i64 {
    optimal_synthesis(30, data)
}

fn solve_part3(data: &Data) -> i64 {
    optimal_synthesis(300, data)
}
