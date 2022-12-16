fn part1() {
    let input = include_str!("../input/day10.input");

    let mut cycle = 0;
    let mut register = 1;
    let mut total_strength = 0;

    for instruction in input.lines() {
        match instruction.split_once(" ") {
            Some((_, change)) => {
                for _ in 0..2 {
                    cycle += 1;
                    if (cycle + 20) % 40 == 0 {
                        println!("CYCLE: {}", cycle);
                        println!("{} * {} = {}", cycle, register, cycle * register);
                        total_strength += cycle * register;
                    }
                }
                register += change.parse::<i32>().unwrap();
            }
            None => {
                cycle += 1;
                if (cycle + 20) % 40 == 0 {
                    println!("CYCLE: {}", cycle);
                    println!("{} * {} = {}", cycle, register, cycle * register);
                    total_strength += cycle * register;
                }
            }
        }
        // println!("CYCLE: {}", cycle);
    }

    println!("Total Strength: {}", total_strength);
}

fn part2() {
    let input = include_str!("../input/day10.input");

    let mut cycle = 0;
    let mut register = 2;

    for instruction in input.lines() {
        match instruction.split_once(" ") {
            Some((_, change)) => {
                for _ in 0..2 {
                    let pixel = (cycle + 1) % 40;
                    cycle += 1;
                    if i32::abs(register - pixel) <= 1 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                    if pixel % 40 == 0 {
                        println!();
                    }
                }
                register += change.parse::<i32>().unwrap();
            }
            None => {
                let pixel = (cycle + 1) % 40;
                cycle += 1;
                if i32::abs(register - pixel) <= 1 {
                    print!("#");
                } else {
                    print!(".");
                }
                if pixel % 40 == 0 {
                    println!();
                }
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
