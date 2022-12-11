use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut counter = 0;

    const WIN_LEN: usize = 4;

    let chars = input.chars().collect::<Vec<_>>();
    for item in chars.windows(WIN_LEN) {
        let st: HashSet<&char> = HashSet::from_iter(item.iter());
        if st.len() != WIN_LEN {
            counter += 1;
        } else {
            break;
        }
    }

    Some(counter + WIN_LEN as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter = 0;

    const WIN_LEN: usize = 14;
    let chars = input.chars().collect::<Vec<_>>();
    for item in chars.windows(WIN_LEN) {
        let st: HashSet<&char> = HashSet::from_iter(item.iter());

        if st.len() != WIN_LEN {
            counter += 1;
        } else {
            break;
        }
    }

    Some(counter + WIN_LEN as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
