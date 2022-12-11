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

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
