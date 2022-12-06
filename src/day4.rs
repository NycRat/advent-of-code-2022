use std::str::FromStr;

struct Assignment {
    pub start: i32,
    pub end: i32,
}

impl FromStr for Assignment {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();

        return Ok(Assignment {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        });
    }
}

fn part1() {
    let input = include_str!("../input/day4.input");
    let sum: i32 = input.lines().map(|assignment_pair| {
        let (assignment1, assignment2) = assignment_pair.split_once(",").unwrap();
        let assignment1: Assignment = assignment1.parse().unwrap();
        let assignment2: Assignment = assignment2.parse().unwrap();
        if assignment1.start >= assignment2.start && assignment1.end <= assignment2.end {
            return 1;
        }
        if assignment2.start >= assignment1.start && assignment2.end <= assignment1.end {
            return 1;
        }
        return 0;
    }).sum();

    println!("ANS: {}", sum);
}

fn part2() {
    let input = include_str!("../input/day4.input");
    let sum: i32 = input.lines().map(|assignment_pair| {
        let (assignment1, assignment2) = assignment_pair.split_once(",").unwrap();
        let assignment1: Assignment = assignment1.parse().unwrap();
        let assignment2: Assignment = assignment2.parse().unwrap();

        if assignment1.start >= assignment2.start && assignment1.start <= assignment2.end {
            return 1;
        }
        if assignment1.end >= assignment2.start && assignment1.end <= assignment2.end {
            return 1;
        }
        if assignment2.start >= assignment1.start && assignment2.start <= assignment1.end {
            return 1;
        }
        if assignment2.end >= assignment1.start && assignment2.end <= assignment1.end {
            return 1;
        }

        return 0;
    }).sum();
    println!("ANS: {}", sum);
}

fn main() {
    part1();
    part2();
}
