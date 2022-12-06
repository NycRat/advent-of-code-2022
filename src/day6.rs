fn solve(len: usize) {
    let input = include_str!("../input/day6.example");
    let mut prev: Vec<char> = Vec::new();
    let mut i = 0;
    for c in input.chars() {
        let mut new_prev = prev.clone();
        let mut ha = 0;
        for e in &prev {
            if e == &c {
                // println!("NOT: {:?}", prev);
                // println!("{}", ha);
                new_prev = prev.get(ha + 1..prev.len()).unwrap().to_vec();
                break;
            }
            ha += 1;
        }
        if prev.len() >= len {
            // println!("{:?}", prev);
            println!("ANS: {}", i);
            break;
        }
        prev = new_prev;
        prev.push(c);
        i += 1;
    }
}

fn part1() {
    solve(4);
}

fn part2() {
    solve(14);
}

fn main() {
    part1();
    part2();
}
