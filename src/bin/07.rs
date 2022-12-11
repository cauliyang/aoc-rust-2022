use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;

pub struct Item {
    pub name: String,
    pub is_dir: bool,
    pub items: Vec<Rc<RefCell<Item>>>,
    pub size: u64,
    pub parent: Link,
}

type Link = Option<Weak<RefCell<Item>>>;

#[derive(Default)]
pub struct ItemBuilder {
    name_: String,
    is_dir_: bool,
    size_: u64,
}

impl ItemBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name_ = name.to_string();
        self
    }

    pub fn is_dir(mut self, is_dir: bool) -> ItemBuilder {
        self.is_dir_ = is_dir;
        self
    }

    pub fn size(mut self, size: u64) -> ItemBuilder {
        self.size_ = size;
        self
    }

    pub fn build(self) -> Rc<RefCell<Item>> {
        Item::new(self.name_.as_str(), self.is_dir_, self.size_)
    }
}

impl Item {
    pub fn builder() -> ItemBuilder {
        ItemBuilder::default()
    }

    pub fn new(name: &str, is_dir: bool, size: u64) -> Rc<RefCell<Item>> {
        if is_dir && size != 0 {
            panic!("Directories cannot have a size");
        }

        Rc::new(RefCell::new(Item {
            name: name.to_string(),
            is_dir,
            items: Vec::new(),
            size,
            parent: None,
        }))
    }

    pub fn item_len(&self) -> usize {
        self.items.len()
    }

    pub fn size(&self) -> usize {
        if self.is_dir {
            self.items.iter().map(|item| item.borrow().size()).sum()
        } else {
            self.size as usize
        }
    }

    pub fn go_parent(&self) -> Option<Rc<RefCell<Item>>> {
        match &self.parent {
            Some(parent) => parent.upgrade(),
            None => None,
        }
    }

    pub fn parent_name(&self) -> String {
        match &self.parent {
            Some(parent) => match parent.upgrade() {
                Some(parent) => parent.borrow().name.clone(),
                None => String::from(""),
            },
            None => String::from(""),
        }
    }

    pub fn sum_directory_by_size(&self, threshold: usize) -> usize {
        let mut sum = 0;

        if self.is_dir {
            let size = self.size();
            if size <= threshold {
                sum += size;
            }

            for item in &self.items {
                sum += item.borrow().sum_directory_by_size(threshold);
            }
        }
        sum
    }

    pub fn filter_directory_by_size(&self, threshold: usize) -> Vec<usize> {
        let mut deleted = Vec::new();

        if self.is_dir {
            let size = self.size();
            if size >= threshold {
                deleted.push(size);
            }

            for item in &self.items {
                deleted.append(&mut item.borrow_mut().filter_directory_by_size(threshold));
            }
        }

        deleted
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in &self.items {
            if item.borrow().is_dir {
                writeln!(f, "d {}", item.borrow().name)?;
                writeln!(f, "{}", item.borrow())?;
            } else {
                writeln!(f, "- {} {}", item.borrow().name, item.borrow().size())?;
            }
        }
        Ok(())
    }
}

pub fn add_item_from_source(parent: &Rc<RefCell<Item>>, name: &str, is_dir: bool, size: u64) {
    let item = Item::new(name, is_dir, size);
    item.borrow_mut().parent = Some(Rc::downgrade(parent));
    parent.borrow_mut().items.push(item);
}

pub fn add_item(parent: &Rc<RefCell<Item>>, item: Rc<RefCell<Item>>) {
    item.borrow_mut().parent = Some(Rc::downgrade(parent));
    parent.borrow_mut().items.push(item);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let item = Item::builder().build();
        assert_eq!(item.borrow().name, "");
        assert!(!item.borrow().is_dir);
        assert_eq!(item.borrow().size, 0);
    }

    #[test]
    fn test_item() {
        let item = Item::builder().name("test").is_dir(true).build();
        assert_eq!(item.borrow().name, "test");
        assert!(item.borrow().is_dir);
        assert_eq!(item.borrow().size, 0);
    }

    #[test]
    fn test_add_item() {
        let item = Item::builder().name("test").is_dir(true).build();

        add_item_from_source(&item, "test2", false, 10);

        let item2 = Item::builder().name("test3").is_dir(false).size(20).build();
        add_item(&item, item2.clone());

        assert_eq!(item2.borrow().parent_name(), "test");
        assert_eq!(item.borrow().item_len(), 2);
        assert_eq!(item.borrow().size(), 30);
    }
}

fn construct(lines: &mut Vec<&str>, item: &Rc<RefCell<Item>>) {
    if let Some(line) = lines.pop() {
        // commands
        if line.starts_with('$') {
            let commands = line.split_whitespace().skip(1).collect::<Vec<_>>();

            match commands.first() {
                Some(&"cd") => {
                    let direc_name = commands.get(1).unwrap();
                    if direc_name == &".." {
                        let parent = item.borrow().go_parent().expect("no parent");
                        construct(lines, &parent);
                    } else {
                        let new_item = Item::builder().name(direc_name).is_dir(true).build();
                        add_item(item, new_item.clone());
                        construct(lines, &new_item);
                    }
                }

                Some(&"ls") => {
                    // add files to directory
                    while let Some(line) = lines.pop() {
                        if line.starts_with('$') {
                            // meet cd command
                            lines.push(line);
                            break;
                        }

                        let file_size = line.split_whitespace().next().unwrap();
                        if file_size != "dir" {
                            let file_name = line.split_whitespace().last().unwrap();
                            add_item_from_source(
                                item,
                                file_name,
                                false,
                                file_size.parse().unwrap(),
                            );
                        }
                    }

                    construct(lines, item);
                }
                _ => {
                    panic!("Unknown command: {}", line)
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = Item::builder().name("/").is_dir(true).build();
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.reverse();
    lines.pop(); // remove first line
    construct(&mut lines, &root);
    let res = root.borrow().sum_directory_by_size(100_000) as u32;
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    const TOTAL_SIZE: usize = 70_000_000;
    const NEED_SIZE: usize = 30_000_000;

    let root = Item::builder().name("/").is_dir(true).build();
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.reverse();
    lines.pop(); // remove first line
    construct(&mut lines, &root);
    let current_size = root.borrow().size();
    let unused_size = TOTAL_SIZE - current_size;
    let need_delete_size = NEED_SIZE - unused_size;

    let deleted = root.borrow_mut().filter_directory_by_size(need_delete_size);
    let res = deleted.iter().min().unwrap();
    Some(*res as u32)
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
