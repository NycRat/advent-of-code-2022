fn get_item_value(c: char) -> u64 {
    if c >= 'a' && c <= 'z' {
        return (c as u32 - 'a' as u32) as u64 + 1;
    } else {
        return (c as u32 - 'A' as u32) as u64 + 27;
    }
}

fn part1() {
    let input = include_str!("../input/day3.example");
    let mut priority_sum = 0;

    for rucksack in input.lines() {
        let mut items: u64 = 0;
        let compartment_len = rucksack.len() / 2;
        for item in rucksack.chars().take(compartment_len) {
            items |= 1 << get_item_value(item);
        }

        for item in rucksack.chars().skip(compartment_len) {
            let item_value = get_item_value(item);
            // println!("{}", item_value);
            if items & (1 << item_value) != 0 {
                priority_sum += item_value;
                break;
            }
        }
    }

    println!("PRIORITY SUM: {}", priority_sum);
}

fn part2() {
    let input = include_str!("../input/day3.input");
    let mut priority_sum = 0;

    let mut lines = input.lines();

    loop {
        let rucksack1 = lines.next().unwrap();
        let rucksack2 = lines.next().unwrap();
        let rucksack3 = lines.next().unwrap();

        let mut items1: u64 = 0;
        for item in rucksack1.chars() {
            items1 |= 1 << get_item_value(item);
        }
        let mut items2: u64 = 0;
        for item in rucksack2.chars() {
            items2 |= 1 << get_item_value(item);
        }
        let mut items3: u64 = 0;
        for item in rucksack3.chars() {
            items3 |= 1 << get_item_value(item);
        }

        let final_item = items1 & items2 & items3;
        priority_sum += f32::log2(final_item as f32) as i32;
        println!("PRIORITY SUM: {}", priority_sum);

    }
}

fn main() {
    part1();
    part2();
}
