use itertools::Itertools;
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (first_start, first_end, second_start, second_end) = line
                    .split(',')
                    .flat_map(|item| {
                        item.split('-')
                            .map(|item| item.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect_tuple()
                    .unwrap();

                if (first_start >= second_start && first_end <= second_end)
                    || (first_start <= second_start && first_end >= second_end)
                {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (first_start, first_end, second_start, second_end) = line
                    .split(',')
                    .flat_map(|item| {
                        item.split('-')
                            .map(|item| item.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect_tuple()
                    .unwrap();

                if first_end < second_start || second_end < first_start {
                    0
                } else {
                    1
                }
            })
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
