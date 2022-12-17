use std::cmp::Ordering;

static INPUT: &str = include_str!("../input/day13.input");

fn split_element(packet: &str) -> Vec<String> {
    if packet == "" {
        return Vec::new();
    }
    let mut elements: Vec<String> = vec![String::new()];
    let mut bracket_count = 0;

    for c in packet.chars() {
        if c == ',' && bracket_count == 0 {
            elements.push(String::new());
            continue;
        }
        if c == '[' {
            bracket_count += 1;
        }
        if c == ']' {
            bracket_count -= 1;
        }
        let back = elements.len() - 1;
        elements[back].push(c);
    }

    return elements;
}

fn is_list(packet: &str) -> bool {
    // println!("Packet: {}", packet);
    return packet.get(0..=0).unwrap() == "[";
}

fn get_order(left: &str, right: &str) -> Ordering {
    if !is_list(left) && !is_list(right) {
        let left_num: i32 = left.parse().unwrap();
        let right_num: i32 = right.parse().unwrap();

        if left_num < right_num {
            return Ordering::Less;
        }
        if left_num > right_num {
            return Ordering::Greater;
        }
    } else {
        let left_elements;
        if is_list(left) {
            left_elements = left.get(1..left.len() - 1).unwrap();
        } else {
            left_elements = left;
        }

        let right_elements;
        if is_list(right) {
            right_elements = right.get(1..right.len() - 1).unwrap();
        } else {
            right_elements = right;
        }

        let left_elements = split_element(left_elements);
        let right_elements = split_element(right_elements);

        for i in 0..usize::min(left_elements.len(), right_elements.len()) {
            let order = get_order(&left_elements[i], &right_elements[i]);
            if order != Ordering::Equal {
                return order;
            }
        }

        if left_elements.len() < right_elements.len() {
            return Ordering::Less;
        }
        if left_elements.len() > right_elements.len() {
            return Ordering::Greater;
        }
    }

    return Ordering::Equal;
}

fn is_right_order(left: &str, right: &str) -> bool {
    return get_order(left, right) == Ordering::Less;
}

fn part1() {
    let ans: usize = INPUT
        .split("\n\n")
        .enumerate()
        .map(|(i, packets)| {
            let (left, right) = packets.split_once("\n").unwrap();
            if is_right_order(left, right) {
                return i + 1;
            }
            return 0;
        })
        .sum();
    println!("Sum: {}", ans);
}

fn part2() {
    let mut packets: Vec<&str> = INPUT.lines().filter(|line| *line != "").collect();
    packets.push("[[2]]");
    packets.push("[[6]]");

    packets.sort_by(|a, b| get_order(a, b));

    let mut index2 = 0;
    let mut index6 = 0;
    for i in 0..packets.len() {
        if packets[i] == "[[2]]" {
            index2 = i + 1;
        }
        if packets[i] == "[[6]]" {
            index6 = i + 1;
        }
    }
    println!("Decoder Key: {}", index2 * index6);
}

fn main() {
    part1();
    part2();
}
