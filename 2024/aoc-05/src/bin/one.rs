use std::collections::{hash_map::Entry, HashMap, HashSet};

fn main() {
    println!("{}", run("inputs/real.txt"));
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
    let updates = line.split(',').map(|s| s.parse::<usize>().unwrap());
    let mut seen = HashSet::new();
    for update in updates {
        if let Some(rhs) = rules.get(&update)
            && !rhs.is_disjoint(&seen)
        {
            return false;
        }
        seen.insert(update);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 143);
    }
}
