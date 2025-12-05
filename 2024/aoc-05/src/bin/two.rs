use std::collections::{HashMap, HashSet, hash_map::Entry};

fn main() {
    println!("{}", run("inputs/real.txt"));
}

#[derive(Clone)]
struct Node {
    value: usize,
    next: Option<Box<Node>>,
}

struct LinkedList {
    len: usize,
    root: Box<Node>,
}

impl LinkedList {
    fn update_next(old: &mut Node, new: &mut Node) {
        new.next = old.next;
        old.next = Some(new);
    }
    fn push_first(&mut self, new: usize) {
        let node = Node {
            value: new,
            next: Some(self.root.clone()),
        };
        self.root = Box::new(node);
        self.len += 1;
    }
    fn push_last(&mut self, new: usize) {
        let mut curr = &mut self.root;
        while let Some(ref mut next) = curr.next {
            curr = next;
        }
        curr.next = Some(Box::new(Node {
            next: None,
            value: new,
        }));
        self.len += 1;
    }
    fn new(value: usize) -> Self {
        LinkedList {
            len: 1,
            root: Box::new(Node { value, next: None }),
        }
    }
}

impl FromIterator<usize> for LinkedList {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut c = MyCollection::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
type Rules = HashMap<usize, HashSet<usize>>;
fn run(path: &str) -> usize {
    let mut updates = false;
    let mut rules = Rules::new();
    let mut sum: usize = 0;
    for line in std::fs::read_to_string(path)
        .expect("path must be valid")
        .lines()
    {
        if line.is_empty() {
            updates = true;
            continue;
        }
        if !updates {
            add_rule(line, &mut rules);
        } else {
            let valid = validate_line(line, &rules);
            if valid {
                let line = line.split(',').collect::<Vec<&str>>();
                let n = line[line.len() / 2].parse::<usize>().unwrap();
                sum += n;
            }
        }
    }
    sum
}

fn add_rule(line: &str, rules: &mut Rules) {
    let mut s = line.split('|');
    let lhs = s.next().unwrap().parse().unwrap();
    let rhs = s.next().unwrap().parse().unwrap();
    match rules.entry(lhs) {
        Entry::Occupied(mut occupied_entry) => {
            occupied_entry.get_mut().insert(rhs);
        }
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(HashSet::from_iter([rhs]));
        }
    }
}

fn validate_line(line: &str, rules: &Rules) -> bool {
    let mut updates = line
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut seen = HashSet::new();
    let mut valid = true;
    for update in updates {
        if let Some(rhs) = rules.get(&update) {
            let intersect = rhs.intersection(&seen).copied().collect::<Vec<usize>>();
            if !intersect.is_empty() {
                valid = false;
                // fix_rule(&mut updates);
            }
        }
        seen.insert(update);
    }
    valid
}

// fn fix_rule(updates: &mut Vec<usize>, infractions: &Vec<usize>) {
//     for n in updates.; {
//         if infractions.contains(n) {
//             updates.splice(range, replace_with)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 123);
    }
}
