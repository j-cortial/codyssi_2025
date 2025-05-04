use std::{
    collections::{HashMap, VecDeque, hash_map::Entry},
    iter::once,
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

type Money = i64;
type Account = &'static str;

struct Transaction {
    from: Account,
    to: Account,
    amount: Money,
}

struct Data {
    initial_balances: HashMap<Account, Money>,
    transactions: Vec<Transaction>,
}

struct Debt {
    to: Account,
    amount: Money,
}

fn parse_input(input: &'static str) -> Data {
    let mut lines = input.lines();

    let mut initial_balances = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let tokens = line.split_once(" HAS ").unwrap();
        initial_balances.insert(tokens.0, tokens.1.parse().unwrap());
    }

    let transactions = lines
        .map(|line| {
            let tokens: Vec<_> = line.split_whitespace().collect();
            Transaction {
                from: tokens[1],
                to: tokens[3],
                amount: tokens[5].parse().unwrap(),
            }
        })
        .collect();

    Data {
        initial_balances,
        transactions,
    }
}

fn solve_part1(data: &Data) -> i64 {
    let mut balances = data.initial_balances.clone();
    for Transaction { from, to, amount } in data.transactions.iter() {
        *balances.get_mut(from).unwrap() -= amount;
        *balances.get_mut(to).unwrap() += amount;
    }
    compute_result(balances)
}

fn solve_part2(data: &Data) -> i64 {
    let mut balances = data.initial_balances.clone();
    for Transaction { from, to, amount } in data.transactions.iter() {
        let amount = *amount.min(balances.get(from).unwrap());
        *balances.get_mut(from).unwrap() -= amount;
        *balances.get_mut(to).unwrap() += amount;
    }
    compute_result(balances)
}

fn solve_part3(data: &Data) -> i64 {
    let mut balances = data.initial_balances.clone();
    let mut debts = HashMap::<Account, VecDeque<Debt>>::new();
    for Transaction { from, to, amount } in data.transactions.iter() {
        let cash_amount = *amount.min(balances.get(from).unwrap());
        if cash_amount < *amount {
            let new_debt = Debt {
                to,
                amount: *amount - cash_amount,
            };
            match debts.entry(from) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().push_back(new_debt);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(VecDeque::from_iter(once(new_debt)));
                }
            };
        }
        *balances.get_mut(from).unwrap() -= cash_amount;
        *balances.get_mut(to).unwrap() += cash_amount;
        let mut debtors = vec![*to];
        while let Some(debtor) = debtors.pop() {
            let maybe_debts = debts.get_mut(debtor);
            if let Some(debts) = maybe_debts {
                while !debts.is_empty() && *balances.get(debtor).unwrap() > 0 {
                    let debt = debts.front_mut().unwrap();
                    let repaid_amount = debt.amount.min(*balances.get(debtor).unwrap());
                    *balances.get_mut(debtor).unwrap() -= repaid_amount;
                    *balances.get_mut(debt.to).unwrap() += repaid_amount;
                    debtors.push(debt.to);
                    debt.amount -= repaid_amount;
                    if debt.amount == 0 {
                        debts.pop_front();
                    }
                }
            }
        }
    }
    compute_result(balances)
}

fn compute_result(balances: HashMap<Account, Money>) -> Money {
    let mut balances: Vec<_> = balances.into_iter().map(|(_, v)| v).collect();
    balances.sort_unstable();
    balances.into_iter().rev().take(3).sum()
}
