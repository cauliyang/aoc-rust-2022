use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first_part: HashSet<char> = HashSet::from_iter(line[..line.len() / 2].chars());
                let second_part: HashSet<char> = HashSet::from_iter(line[line.len() / 2..].chars());
                let common_char = first_part.intersection(&second_part).next().unwrap();

                if common_char.is_ascii_lowercase() {
                    *common_char as u32 - 96
                } else {
                    *common_char as u32 - 38
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    use itertools::Itertools;

    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|mut chunk| {
                let first = chunk.next().unwrap();
                let second = chunk.next().unwrap();
                let third = chunk.next().unwrap();
                first
                    .chars()
                    .find(|item| second.contains(*item) && third.contains(*item))
                    .unwrap()
            })
            .map(|item| {
                if item.is_ascii_lowercase() {
                    item as u32 - 96
                } else {
                    item as u32 - 38
                }
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
