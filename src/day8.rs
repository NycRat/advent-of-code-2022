fn part1() {
    let input = include_str!("../input/day8.input");
    let mut trees: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars())
        .map(|trees| {
            trees
                .map(|tree| tree.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let width = trees[0].len();
    let height = trees.len();

    let mut visible = (width * 2 + height * 2 - 4) as i32;

    for x in 1..width - 1 {
        let mut cur_tallest = trees[x][0];
        for y in 1..height - 1 {
            let tree = &mut trees[x][y];
            if i32::abs(*tree) > cur_tallest {
                cur_tallest = i32::abs(*tree);
                if *tree > 0 {
                    println!("Visible at3: ({}, {}) {}", x, y, tree);
                    *tree = -*tree;
                    visible += 1;
                }
            }
        }
    }

    for y in 1..height - 1 {
        let mut cur_tallest = trees[0][y];
        for x in 1..width - 1 {
            let tree = &mut trees[x][y];
            if i32::abs(*tree) > cur_tallest {
                cur_tallest = i32::abs(*tree);
                if *tree > 0 {
                    println!("Visible at2: ({}, {}) {}", x, y, tree);
                    *tree = -*tree;
                    visible += 1;
                }
            }
        }
    }

    for x in 1..width - 1 {
        let mut cur_tallest = trees[x][height - 1];
        for y in (1..height - 1).rev() {
            let tree = &mut trees[x][y];
            if i32::abs(*tree) > cur_tallest {
                cur_tallest = i32::abs(*tree);
                if *tree > 0 {
                    println!("Visible at1: ({}, {}) {}", x, y, tree);
                    *tree = -*tree;
                    visible += 1;
                }
            }
        }
    }

    for y in 1..height - 1 {
        let mut cur_tallest = trees[width - 1][y];
        println!("CUR: {}", cur_tallest);
        for x in (1..width - 1).rev() {
            let tree = &mut trees[x][y];
            if i32::abs(*tree) > cur_tallest {
                cur_tallest = i32::abs(*tree);
                if *tree > 0 {
                    println!("Visible at0: ({}, {}) {}", x, y, tree);
                    *tree = -*tree;
                    visible += 1;
                }
            }
        }
    }

    println!("{:?}", trees);

    println!("Trees Visible: {}", visible);
}

fn part2() {
    // let input = include_str!("../input/day8.example");
    let input = include_str!("../input/day8.input");

    let trees: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|trees| trees.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut best_scenic_score = 0;

    let width = trees[0].len();
    let height = trees.len();

    for y in 0..height {
        for x in 0..width {
            let mut a = 0;
            let mut b = 0;
            let mut c = 0;
            let mut d = 0;
            let cur_tree = trees[y][x];
            for xi in (0..x).rev() {
                a += 1;
                if cur_tree <= trees[y][xi] {
                    break;
                }
            }
            for xi in x+1..width {
                b += 1;
                if cur_tree <= trees[y][xi] {
                    break;
                }
            }
            for yi in (0..y).rev() {
                c += 1;
                if cur_tree <= trees[yi][x] {
                    break;
                }
            }
            for yi in y+1..height {
                d += 1;
                if cur_tree <= trees[yi][x] {
                    break;
                }
            }
            println!("({}, {}): {}", x, y, a * b * c * d);
            best_scenic_score = best_scenic_score.max(a * b * c * d);
        }
    }

    println!("BEST SCENIC SCORE: {}", best_scenic_score);
}

fn main() {
    // part1();
    part2();
}
