static INPUT: &str = include_str!("../input/day15.input");

#[derive(PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

fn get_distance(a: &Coord, b: &Coord) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

fn get_sensor_and_beacons() -> Vec<(Coord, Coord)> {
    return INPUT
        .lines()
        .map(|line| {
            let stuff: Vec<&str> = line.split("=").collect();
            let sensor = Coord {
                x: stuff[1].split_once(",").unwrap().0.parse().unwrap(),
                y: stuff[2].split_once(":").unwrap().0.parse().unwrap(),
            };
            let beacon = Coord {
                x: stuff[3].split_once(",").unwrap().0.parse().unwrap(),
                y: stuff[4].parse().unwrap(),
            };

            // println!(
            //     "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            //     sensor.x, sensor.y, beacon.x, beacon.y
            // );

            return (sensor, beacon);
        })
        .collect();
}

fn part1() {
    let sensors_and_beacons = get_sensor_and_beacons();

    // let row = 10;
    let row = 2000000;
    let mut ans = 0;
    for x in -10000000..10000000 {
        for (sensor, beacon) in &sensors_and_beacons {
            let pos = Coord { x, y: row };

            if &pos == beacon {
                continue;
            }

            let closest = get_distance(sensor, beacon);
            if get_distance(&pos, sensor) <= closest {
                ans += 1;
                break;
            }
        }
    }
    println!("Ans: {}", ans);
}

fn part2() {
    let sensors_and_beacons = get_sensor_and_beacons();
    let max_range = 4000000;

    for x in 0..max_range {
        let mut y = 0;
        while y < max_range {
            let mut max_far = -1;
            for (sensor, beacon) in &sensors_and_beacons {
                let pos = Coord { x, y };

                let closest = get_distance(sensor, beacon);
                let cur_distance = get_distance(&pos, sensor);
                if cur_distance <= closest {
                    max_far = max_far.max(closest - cur_distance).abs();
                }
            }

            if max_far == -1 {
                println!("Ans: {}", x as u128 * 4000000 + y as u128);
                return;
            }

            y += max_far.max(1);
        }
    }
}

fn main() {
    part1();
    part2();
}
