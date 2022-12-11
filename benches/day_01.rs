use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn part_one_ori(input: &str) -> Option<u32> {
    let mut max_value = 0;
    let mut summary = 0;

    for line in input.lines() {
        if line.is_empty() {
            if summary > max_value {
                max_value = summary;
            }
            summary = 0;
        } else {
            summary += line.parse::<u32>().unwrap();
        }
    }

    Some(max_value)
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
}

pub fn part_two_ori(input: &str) -> Option<u32> {
    let mut res = [0; 3];
    let mut summary = 0;

    for line in input.lines() {
        if line.is_empty() {
            if summary > res[0] {
                res[2] = res[1];
                res[1] = res[0];
                res[0] = summary;
            } else if summary > res[1] {
                res[2] = res[1];
                res[1] = summary;
            } else if summary > res[2] {
                res[2] = summary;
            } else {
                // do nothing
            }

            summary = 0;
        } else {
            summary += line.parse::<u32>().unwrap();
        }
    }
    Some(res.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    res.sort_unstable_by(|a, b| b.cmp(a));

    Some(res.into_iter().take(3).sum())
}

fn test1() {
    let input = &advent_of_code::read_file("inputs", 1);
    part_one(input);
}

fn test2() {
    let input = &advent_of_code::read_file("inputs", 1);
    part_one_ori(input);
}

fn test3() {
    let input = &advent_of_code::read_file("inputs", 1);
    part_two(input);
}

fn test4() {
    let input = &advent_of_code::read_file("inputs", 1);
    part_two_ori(input);
}

pub fn criterion_part_one(c: &mut Criterion) {
    c.bench_function("part_one", |b| b.iter(|| test1()));
}

pub fn criterion_part_one_ori(c: &mut Criterion) {
    c.bench_function("part_one_ori", |b| b.iter(|| test2()));
}

pub fn criterion_part_two(c: &mut Criterion) {
    c.bench_function("part_two", |b| b.iter(|| test3()));
}

pub fn criterion_part_two_ori(c: &mut Criterion) {
    c.bench_function("part_two_ori", |b| b.iter(|| test4()));
}

criterion_group!(
    benches,
    criterion_part_one,
    criterion_part_one_ori,
    criterion_part_two,
    criterion_part_two_ori
);
criterion_main!(benches);
