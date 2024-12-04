use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve() {
    let (mut left, mut right) = parse_file("inputs/input01");

    println!("===== DAY 01 =====");
    println!("Part 1: {}", part1(&mut left, &mut right));
    println!("Part 2: {}", part2(&mut left, &mut right));
}

fn part1(left: &mut [u64], right: &mut [u64]) -> u64 {
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .fold(0, |n, (a, b)| n + a.abs_diff(*b))
}

fn part2(left: &mut [u64], right: &mut [u64]) -> u64 {
    let mut hm: HashMap<u64, u64> = HashMap::new();
    right.iter().for_each(|v| {
        let counter = hm.entry(*v).or_insert(0);
        *counter += 1
    });

    left.iter().fold(0, |n, v| n + v * hm.get(v).unwrap_or(&0))
}

fn parse_file<P: AsRef<Path>>(path: P) -> (Vec<u64>, Vec<u64>) {
    let file = File::open(path).unwrap();

    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    BufReader::new(file).lines().into_iter().for_each(|line| {
        if let Some((l, r)) = line.unwrap().split_once(' ') {
            left.push(l.trim().parse().unwrap());
            right.push(r.trim().parse().unwrap());
        }
    });

    (left, right)
}
