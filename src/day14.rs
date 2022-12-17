use std::collections::HashSet;

static INPUT: &str = include_str!("../input/day14.input");

fn part1() {
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut lowest = 0; // higher is lower

    for line in INPUT.lines() {
        let points: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|point| {
                let (x, y) = point.split_once(",").unwrap();
                return (x.parse().unwrap(), y.parse().unwrap());
            })
            .collect();

        for i in 1..points.len() {
            let start = &points[i - 1];
            let end = &points[i];

            let start_x = i32::min(start.0, end.0);
            let end_x = i32::max(start.0, end.0);
            let start_y = i32::min(start.1, end.1);
            let end_y = i32::max(start.1, end.1);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    lowest = lowest.max(y);
                    rocks.insert((x, y));
                }
            }
        }
    }

    let mut sand = (500, 0);
    let mut ans = 0;

    loop {
        if sand.1 > lowest {
            break;
        }
        if !rocks.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
            continue;
        }
        if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
            continue;
        }
        if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
            continue;
        }

        rocks.insert(sand);
        ans += 1;
        sand = (500, 0);
    }
    println!("Ans: {}", ans);
}

fn part2() {
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut lowest = 0; // higher is lower

    for line in INPUT.lines() {
        let points: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|point| {
                let (x, y) = point.split_once(",").unwrap();
                return (x.parse().unwrap(), y.parse().unwrap());
            })
            .collect();

        for i in 1..points.len() {
            let start = &points[i - 1];
            let end = &points[i];

            let start_x = i32::min(start.0, end.0);
            let end_x = i32::max(start.0, end.0);
            let start_y = i32::min(start.1, end.1);
            let end_y = i32::max(start.1, end.1);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    lowest = lowest.max(y);
                    rocks.insert((x, y));
                }
            }
        }
    }

    let mut sand = (500, 0);
    let mut ans = 0;

    loop {
        if rocks.contains(&(500, 0)) {
            break;
        }
        if sand.1 + 1 >= lowest + 2 {
            rocks.insert(sand);
            ans += 1;
            sand = (500, 0);
            continue;
        }
        if !rocks.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
            continue;
        }
        if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
            continue;
        }
        if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
            continue;
        }

        rocks.insert(sand);
        ans += 1;
        sand = (500, 0);
    }
    println!("Ans: {}", ans);
}

fn main() {
    part1();
    part2();
}
