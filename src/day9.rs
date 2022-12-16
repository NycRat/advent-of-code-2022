use std::collections::HashSet;

fn is_one_away(head: &[i32; 2], tail: &[i32; 2]) -> bool {
    if (head[0] - tail[0]).abs() > 1 {
        return false;
    }
    if (head[1] - tail[1]).abs() > 1 {
        return false;
    }
    return true;
}

fn do_move(direction: &str, head: &mut [i32; 2], tail: &mut [i32; 2]) -> String {
    match direction {
        "U" => {
            head[1] -= 1;
            if !is_one_away(&head, &tail) {
                // tail[1] -= 1;
                // tail[0] = head[0];
                if head[0] > tail[0] {
                    return "UR".to_owned();
                }
                if head[0] < tail[0] {
                    return "UL".to_owned();
                }
                return direction.to_owned();
            }
            return "".to_owned();
        }
        "D" => {
            head[1] += 1;
            if !is_one_away(&head, &tail) {
                // tail[1] += 1;
                // tail[0] = head[0];
                if head[0] > tail[0] {
                    return "DR".to_owned();
                }
                if head[0] < tail[0] {
                    return "DL".to_owned();
                }
                return direction.to_owned();
            }
            return "".to_owned();
        }
        "L" => {
            head[0] -= 1;
            if !is_one_away(&head, &tail) {
                // tail[0] -= 1;
                // tail[1] = head[1];
                if head[1] < tail[1] {
                    return "UL".to_owned();
                }
                if head[1] > tail[1] {
                    return "DL".to_owned();
                }
                return direction.to_owned();
            }
            return "".to_owned();
        }
        "R" => {
            head[0] += 1;
            if !is_one_away(&head, &tail) {
                // tail[0] += 1;
                // tail[1] = head[1];
                if head[1] < tail[1] {
                    return "UR".to_owned();
                }
                if head[1] > tail[1] {
                    return "DR".to_owned();
                }
                return direction.to_owned();
            }
            return "".to_owned();
        }
        "UR" => {
            head[0] += 1;
            head[1] -= 1;

            if !is_one_away(&head, &tail) {
                let mut new_direction = String::from("");
                if head[1] < tail[1] {
                    // tail[1] -= 1;
                    new_direction += "U";
                }
                if head[0] > tail[0] {
                    // tail[0] += 1;
                    new_direction += "R";
                }
                return new_direction;
            }
            return "".to_owned();
        }
        "DR" => {
            head[0] += 1;
            head[1] += 1;
            if !is_one_away(&head, &tail) {
                let mut new_direction = String::from("");
                if head[1] > tail[1] {
                    // tail[1] += 1;
                    new_direction += "D";
                }
                if head[0] > tail[0] {
                    // tail[0] += 1;
                    new_direction += "R";
                }
                return new_direction;
            }
            return "".to_owned();
        }
        "UL" => {
            head[0] -= 1;
            head[1] -= 1;
            if !is_one_away(&head, &tail) {
                let mut new_direction = String::from("");
                if head[1] < tail[1] {
                    new_direction += "U";
                }
                if head[0] < tail[0] {
                    new_direction += "L";
                }
                return new_direction;
            }
            return "".to_owned();
        }
        "DL" => {
            head[0] -= 1;
            head[1] += 1;
            if !is_one_away(&head, &tail) {
                let mut new_direction = String::from("");
                if head[1] > tail[1] {
                    // tail[1] += 1;
                    new_direction += "D";
                }
                if head[0] < tail[0] {
                    // tail[0] -= 1;
                    new_direction += "L";
                }
                return new_direction;
            }
            return "".to_owned();
        }
        "" => {
            return "".to_owned();
        }
        _ => {
            unreachable!();
        }
    }
}

fn part1() {
    let input = include_str!("../input/day9.input");

    let mut head_pos = [0, 0];
    let mut tail_pos = [0, 0];

    let mut visited: HashSet<[i32; 2]> = HashSet::new();
    // visited.insert([0,0]);

    for motion in input.lines() {
        // input.lines().map(|motion|{
        let (direction, distance) = motion.split_once(" ").unwrap();

        let distance: i32 = distance.parse().unwrap();

        for _ in 0..distance {
            do_move(direction, &mut head_pos, &mut tail_pos);
            visited.insert(tail_pos);
        }
    }

    println!("Visited: {}", visited.len());
}
fn part2() {
    let input = include_str!("../input/day9.input");

    let mut knots = vec![
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
    ];

    let num_knots = knots.len();

    let mut visited: HashSet<[i32; 2]> = HashSet::new();
    // visited.insert([0,0]);

    for motion in input.lines() {
        // input.lines().map(|motion|{
        let (original_direction, distance) = motion.split_once(" ").unwrap();

        let distance: i32 = distance.parse().unwrap();

        println!("{} {}", original_direction, distance);
        for _ in 0..distance {
            let mut direction = original_direction.to_owned();
            for i in 1..num_knots {
                let mut head = knots[i - 1];
                let mut tail = knots[i];

                // println!("BEFORE: {:?} {:?}", head, tail);
                direction = do_move(direction.as_str(), &mut head, &mut tail);

                knots[i - 1] = head;
                knots[i] = tail;
                // do_move(direction, &mut knots[i], &mut knots[i-1]);
            }
            let mut head = knots[num_knots - 1];
            let mut tail = [0, 0];
            do_move(&direction, &mut head, &mut tail);
            knots[num_knots - 1] = head;
            println!("{:?}", knots);
            visited.insert(knots[num_knots - 1]);
        }
    }

    println!("Visited: {}", visited.len());
}
fn main() {
    // part1();
    part2();
}
