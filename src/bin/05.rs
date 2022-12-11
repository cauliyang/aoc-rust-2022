use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut stacks_lines = input
        .lines()
        .take_while(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    let num_lines = stacks_lines.len();

    let mut stacks = HashMap::new();
    stacks_lines.pop();
    for line in stacks_lines.iter().rev() {
        for (id, ch) in line[1..].bytes().step_by(4).enumerate() {
            if ch != b' ' {
                stacks.entry(id + 1).or_insert(Vec::new()).push(ch as char);
            }
        }
    }

    for ops in input.lines().skip(num_lines + 1) {
        let (_, num, _, source, _, target) = ops.split_whitespace().collect_tuple().unwrap();
        // move
        // 1. remove from source

        let num = num.parse::<usize>().unwrap();
        let source = source.parse::<usize>().unwrap();
        let target = target.parse::<usize>().unwrap();

        let souce_stack: &mut Vec<char> = stacks.get_mut(&source).unwrap();
        let mut source_items = souce_stack.split_off(souce_stack.len() - num);
        source_items.reverse();

        // 2. add to target

        let target_stack: &mut Vec<char> = stacks.get_mut(&target).unwrap();
        target_stack.extend(source_items);
    }

    let mut res = "".to_string();
    for i in 1..=stacks.len() {
        let stack = stacks.get_mut(&i).unwrap().pop().unwrap();
        res.push(stack);
    }
    println!("{}", res);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stacks_lines = input
        .lines()
        .take_while(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    let num_lines = stacks_lines.len();

    let mut stacks = HashMap::new();
    stacks_lines.pop();
    for line in stacks_lines.iter().rev() {
        for (id, ch) in line[1..].bytes().step_by(4).enumerate() {
            if ch != b' ' {
                stacks.entry(id + 1).or_insert(Vec::new()).push(ch as char);
            }
        }
    }

    for ops in input.lines().skip(num_lines + 1) {
        let (_, num, _, source, _, target) = ops.split_whitespace().collect_tuple().unwrap();
        // move
        // 1. remove from source

        let num = num.parse::<usize>().unwrap();
        let source = source.parse::<usize>().unwrap();
        let target = target.parse::<usize>().unwrap();

        let souce_stack: &mut Vec<char> = stacks.get_mut(&source).unwrap();
        let source_items = souce_stack.split_off(souce_stack.len() - num);

        // 2. add to target

        let target_stack: &mut Vec<char> = stacks.get_mut(&target).unwrap();
        target_stack.extend(source_items);
    }

    let mut res = "".to_string();
    for i in 1..=stacks.len() {
        let stack = stacks.get_mut(&i).unwrap().pop().unwrap();
        res.push(stack);
    }
    println!("{}", res);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
