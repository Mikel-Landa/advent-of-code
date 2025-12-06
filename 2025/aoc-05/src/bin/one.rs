fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let file = std::fs::read_to_string(path).expect("path must exist");
    let mut lines = file.lines().into_iter();
    let mut fresh_ranges = Vec::new();
    let mut sum = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once('-').unwrap();
        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
        fresh_ranges.push((a, b));
    }

    for line in lines {
        let id: usize = line.parse().unwrap();
        for (x, y) in &fresh_ranges {
            if id >= *x && id <= *y {
                sum += 1;
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 3)
    }
}
