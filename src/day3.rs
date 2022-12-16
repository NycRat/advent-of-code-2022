fn get_item_value(c: char) -> u64 {
    if c >= 'a' && c <= 'z' {
        return (c as u32 - 'a' as u32) as u64 + 1;
    } else {
        return (c as u32 - 'A' as u32) as u64 + 27;
    }
}

fn part1() {
    let input = include_str!("../input/day3.input");
    let mut priority_sum = 0;

    for rucksack in input.lines() {
        let mut items: u64 = 0;
        let compartment_len = rucksack.len() / 2;
        for item in rucksack.chars().take(compartment_len) {
            items |= 1 << get_item_value(item);
        }

        for item in rucksack.chars().skip(compartment_len) {
            let item_value = get_item_value(item);
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

    let mut rucksacks = input.lines();

    loop {
        let mut common_item = u64::MAX;
        for _ in 0..3 {
            let rucksack = rucksacks.next();
            if rucksack.is_none() {
                println!("PRIORITY SUM: {}", priority_sum);
                return;
            }
            let mut items: u64 = 0;
            for item in rucksack.unwrap().chars() {
                items |= 1 << get_item_value(item);
            }
            common_item &= items;
        }

        priority_sum += f32::log2(common_item as f32) as i32;
    }
}

fn main() {
    part1();
    part2();
}
