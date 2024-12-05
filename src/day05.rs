use std::{
    collections::HashSet,
    fs::{self},
    path::Path,
};

pub fn solve() {
    let (mut order, mut updates) = parse_file("inputs/input05");

    println!("===== DAY 05 =====");
    println!("Part 1: {}", part1(&mut order, &mut updates));
    // println!("Part 2: {}", part2(&mut left, &mut right));
}

fn part1(order: &Vec<(u8, u8)>, updates: &mut Vec<Vec<u8>>) -> u64 {
    let mut res: u64 = 0;
    for update in updates {
        let mut is_ordered = true;

        let mut encountered = HashSet::new();

        for val in update.iter() {
            // There cannot be val | <encountered> in our order
            for (before, after) in order {
                if *before == *val {
                    if encountered.contains(after) {
                        is_ordered = false;
                        break;
                    }
                }
                encountered.insert(*val);
            }

            if !is_ordered {
                break;
            }
        }

        if is_ordered {
            res += update[update.len() / 2] as u64;
        }
    }

    res
}

fn parse_file<P: AsRef<Path>>(path: P) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines();

    let order: Vec<(u8, u8)> = lines
        .clone()
        .filter(|line| line.contains('|'))
        .filter_map(|l| {
            if let Some((l, r)) = l.split_once('|') {
                Some((l.to_owned(), r.to_owned()))
            } else {
                None
            }
        })
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let updates: Vec<Vec<u8>> = lines
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|job| job.parse().unwrap()).collect())
        .collect();

    (order, updates)
}
