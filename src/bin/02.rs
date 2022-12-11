use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let score_map = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let win_map = HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]);
    let draw_map = HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]);

    Some(
        input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let char_a = chars.next().unwrap();
                let char_b = chars.last().unwrap();
                if win_map.get(&char_a).unwrap() == &char_b {
                    score_map[&char_b] + 6
                } else if draw_map.get(&char_a).unwrap() == &char_b {
                    score_map[&char_b] + 3
                } else {
                    score_map[&char_b]
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let score_map = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let win_map = HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]);
    let draw_map = HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]);
    let lose_map = HashMap::from([('A', 'Z'), ('B', 'X'), ('C', 'Y')]);

    Some(
        input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let char_a = chars.next().unwrap();
                let char_b = chars.last().unwrap();

                if char_b == 'X' {
                    score_map[lose_map.get(&char_a).unwrap()]
                } else if char_b == 'Y' {
                    score_map[draw_map.get(&char_a).unwrap()] + 3
                } else {
                    score_map[win_map.get(&char_a).unwrap()] + 6
                }
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
