use std::{result, str::FromStr};

#[derive(Debug)]
struct Move {
    amount: i32,
    start: usize,
    end: usize,
}

impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (amount, positions) = s.split_once(" from ").unwrap();
        let (start, end) = positions.split_once(" to ").unwrap();
        let (_, amount) = amount.split_once(" ").unwrap();

        return Ok(Move {
            amount: amount.parse().unwrap(),
            start: start.parse::<usize>().unwrap()-1,
            end: end.parse::<usize>().unwrap()-1,
        });
    }
}

fn parse_crates(crates_str: &str) -> Vec<Vec<&str>> {
    let mut crates: Vec<Vec<&str>> = Vec::new();


    let asd: String = crates_str.lines().rev().take(1).collect();
    let num_crates = (asd.len()+1) / 4;
    println!("LEN: {}", num_crates);

    for i in 0..num_crates {
        crates.push(Vec::new());
        println!("{}", i);
    }
    for line in crates_str.lines().rev().skip(1) {
        let mut i = 0;
        while i < line.len() && i < num_crates {
            let pos = i * 4 + 1;
            let c = line.get(pos..=pos).unwrap();
            if c != " " {
                crates[i].push(c);
            }
            i += 1;
        }
    }
    return crates;
}

fn part1() {
    // let input = include_str!("../input/day5.example");
    let input = include_str!("../input/day5.input");

    let (crates, moves) = input.split_once("\n\n").unwrap();

    let mut crates = parse_crates(crates);

    let moves: Vec<Move> = moves
        .lines()
        .map(str::parse)
        .map(result::Result::unwrap)
        .collect();

    for movee in moves {
        let mut moved_crates = Vec::new();
        for _ in 0..movee.amount {
            moved_crates.push(crates[movee.start].pop().unwrap());
        }
        for cratee in moved_crates {
            println!("{}", movee.end);
            crates[movee.end].push(cratee);
        }
    }

    print!("CRATES: ");
    for crate_stack in &crates {
        print!("{}", crate_stack[crate_stack.len()-1]);
    }
    println!("");
    // println!("{:?}", crates);
    // println!("{:?}", moves);
}

fn part2() {
    // let input = include_str!("../input/day5.example");
    let input = include_str!("../input/day5.input");

    let (crates, moves) = input.split_once("\n\n").unwrap();

    let mut crates = parse_crates(crates);

    let moves: Vec<Move> = moves
        .lines()
        .map(str::parse)
        .map(result::Result::unwrap)
        .collect();

    for movee in moves {
        let mut moved_crates = Vec::new();
        for _ in 0..movee.amount {
            moved_crates.push(crates[movee.start].pop().unwrap());
        }
        for cratee in moved_crates.into_iter().rev() {
            println!("{}", movee.end);
            crates[movee.end].push(cratee);
        }
    }

    print!("CRATES: ");
    for crate_stack in &crates {
        print!("{}", crate_stack[crate_stack.len()-1]);
    }
    println!("");
}

fn main() {
    part1();
    part2();
}
