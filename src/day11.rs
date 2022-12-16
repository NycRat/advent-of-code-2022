use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Addition(u128),
    Multiplication(u128),
    Square(),
}

#[derive(Debug)]
struct Test {
    condition: u128,
    monkey_true: usize,
    monkey_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: Test,
    inspected: u32,
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stuff: Vec<&str> = s.lines().skip(1).collect();

        let items = stuff[0].split_once(": ").unwrap().1;
        let items: Vec<u128> = items
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = stuff[1].split_once("old ").unwrap().1;
        let operation = match operation.split_once(" ").unwrap() {
            ("+", "old") => Operation::Multiplication(2),
            ("*", "old") => Operation::Square(),
            ("+", num) => Operation::Addition(num.parse().unwrap()),
            ("*", num) => Operation::Multiplication(num.parse().unwrap()),
            (_, _) => {
                unreachable!();
            }
        };

        let test = Test {
            condition: stuff[2].split_once("by ").unwrap().1.parse().unwrap(),
            monkey_true: stuff[3].split_once("monkey ").unwrap().1.parse().unwrap(),
            monkey_false: stuff[4].split_once("monkey ").unwrap().1.parse().unwrap(),
        };

        let inspected = 0;

        return Ok(Monkey {
            items,
            operation,
            test,
            inspected,
        });
    }
}

impl Monkey {
    fn apply_operation(&mut self, i: usize) {
        match &self.operation {
            Operation::Addition(n) => {
                self.items[i] += n;
            }
            Operation::Multiplication(n) => {
                self.items[i] *= n;
            }
            Operation::Square() => {
                self.items[i] *= self.items[i];
            }
        }
    }
}

fn get_monkey_business(rounds: i32, divide_worry: bool) -> u128 {
    let input = include_str!("../input/day11.input");
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let mut common_condition = 1;
    for m in &monkeys {
        common_condition *= m.test.condition;
    }

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                monkeys[i].inspected += 1;
                monkeys[i].apply_operation(j);
                if divide_worry {
                    monkeys[i].items[j] /= 3;
                } else {
                    monkeys[i].items[j] %= common_condition;
                }
                if monkeys[i].items[j] % monkeys[i].test.condition == 0 {
                    let item = monkeys[i].items[j];
                    let throw_index = monkeys[i].test.monkey_true;
                    monkeys[throw_index].items.push(item);
                } else {
                    let item = monkeys[i].items[j];
                    let throw_index = monkeys[i].test.monkey_false;
                    monkeys[throw_index].items.push(item);
                }
            }
            monkeys[i].items.clear();
        }
    }
    let mut max1 = 0;
    let mut max2 = 0;
    for monkey in &monkeys {
        if monkey.inspected > max2 {
            max2 = monkey.inspected;
        }
        if max2 > max1 {
            let t = max1;
            max1 = max2;
            max2 = t;
        }
    }

    return max1 as u128 * max2 as u128;
}

fn part1() {
    println!("Part 1: {}", get_monkey_business(20, true));
}

fn part2() {
    println!("Part 2: {}", get_monkey_business(10000, false));
}

fn main() {
    part1();
    part2();
}
