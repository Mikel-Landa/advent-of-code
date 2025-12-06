use std::collections::HashSet;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let file = std::fs::read_to_string(path).expect("path must exist");
    let mut lines = file.lines().into_iter();
    let mut fresh_ranges: HashSet<(usize, usize)> = HashSet::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once('-').unwrap();
        let (mut a, mut b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
        let mut changed = Vec::new();
        for i in fresh_ranges.iter() {
            let x = (*i).0;
            let y = (*i).1;
            if (a >= x && a <= y) || (b >= x && a < x) {
                a = x.min(a);
                b = y.max(b);
                changed.push((x, y));
            }
        }
        for i in changed {
            fresh_ranges.remove(&i);
        }
        fresh_ranges.insert((a, b));
    }

    fresh_ranges
        .into_iter()
        .fold(0, |acc, (x, y)| acc + 1 + y - x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 14)
    }
}
