use std::str::FromStr;

static INPUT: &str = include_str!("../input/day12.input");

struct Hills {
    heights: Vec<Vec<i32>>,
    steps: Vec<Vec<i32>>,
}

impl FromStr for Hills {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights: Vec<Vec<i32>> = s
            .lines()
            .map(|row| {
                row.chars()
                    .map(|height| match height {
                        'S' => 0,
                        'E' => 27,
                        _ => height as i32 - 'a' as i32 + 1,
                    })
                    .collect()
            })
            .collect();

        let hills_height = heights.len();
        let hills_width = heights[0].len();

        return Ok(Hills {
            heights,
            steps: vec![vec![i32::MAX; hills_width]; hills_height],
        });
    }
}

impl Hills {
    fn width(&self) -> usize {
        return self.heights[0].len();
    }
    fn height(&self) -> usize {
        return self.heights.len();
    }
}

fn should_visit(hills: &mut Hills, pos: (usize, usize), new_pos: (usize, usize)) -> bool {
    let old_height = hills.heights[pos.1][pos.0];
    let new_height = hills.heights[new_pos.1][new_pos.0];
    if old_height == 25 && new_height == 27 {
        return true;
    }
    if old_height + 1 < new_height {
        return false;
    }
    return true;
}

fn rec(hills: &mut Hills, pos: (usize, usize), steps: i32) -> i32 {
    let (x, y) = pos;

    if steps >= hills.steps[y][x] {
        return i32::MAX;
    }

    hills.steps[y][x] = steps;

    if hills.heights[y][x] == 27 {
        return steps;
    }

    let mut min_steps = i32::MAX;

    let steps = steps + 1;

    if x < hills.width() - 1 {
        let new_pos = (x + 1, y);
        if should_visit(hills, pos, new_pos) {
            min_steps = min_steps.min(rec(hills, new_pos, steps));
        }
    }
    if x > 0 {
        let new_pos = (x - 1, y);
        if should_visit(hills, pos, new_pos) {
            min_steps = min_steps.min(rec(hills, new_pos, steps));
        }
    }
    if y < hills.height() - 1 {
        let new_pos = (x, y + 1);
        if should_visit(hills, pos, new_pos) {
            min_steps = min_steps.min(rec(hills, new_pos, steps));
        }
    }
    if y > 0 {
        let new_pos = (x, y - 1);
        if should_visit(hills, pos, new_pos) {
            min_steps = min_steps.min(rec(hills, new_pos, steps));
        }
    }

    return min_steps;
}

fn part1() {
    let mut hills: Hills = INPUT.parse().unwrap();
    for y in 0..hills.height() {
        for x in 0..hills.width() {
            if hills.heights[y][x] == 0 {
                println!("Minimum steps: {}", rec(&mut hills, (x, y), 0));
                return;
            }
        }
    }
}

fn part2() {
    let mut hills: Hills = INPUT.parse().unwrap();
    let mut min_steps = i32::MAX;
    for y in 0..hills.height() {
        for x in 0..hills.width() {
            if hills.heights[y][x] == 1 {
                min_steps = min_steps.min(rec(&mut hills, (x, y), 0));
            }
        }
    }
    println!("Minimum steps: {}", min_steps);
}

fn main() {
    part1();
    part2();
}
