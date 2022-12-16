use std::collections::HashMap;

fn get_dirs() -> HashMap<String, u32> {
    let input = include_str!("../input/day7.input");
    let mut dirs: HashMap<String, u32> = HashMap::new();

    let mut prev_dirs: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.get(..4).unwrap_or("") == "$ cd" {
            let dir = line.get(5..).unwrap();
            if dir == ".." {
                prev_dirs.pop();
            } else {
                prev_dirs.push(dir);
            }
        } else if line.get(..3).unwrap_or(line) != "dir" && line.get(..4).unwrap_or(line) != "$ ls"
        {
            for i in 1..=prev_dirs.len() {
                let path = prev_dirs[0..i].join("/");
                let size = dirs.get(&path).unwrap_or(&0);
                let (new_size, _) = line.split_once(" ").unwrap();
                dirs.insert(path, new_size.parse::<u32>().unwrap() + size);
            }
        }
    }
    return dirs;
}

fn part1() {
    println!(
        "ANS: {}",
        get_dirs()
            .iter()
            .map(|(_, size)| if size < &100000 { *size } else { 0 })
            .sum::<u32>()
    );
}
fn part2() {
    let dirs = get_dirs();
    let used = 70000000 - dirs.get("/").unwrap();
    println!(
        "ANS: {}",
        dirs.iter()
            .map(|(_, size)| if size + used >= 30000000 {
                *size
            } else {
                u32::MAX
            })
            .min()
            .unwrap()
    );
}
fn main() {
    part1();
    part2();
}
