use std::collections::HashMap;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let file = std::fs::read_to_string(path).expect("path must exist");
    let mut lines = file.lines();

    let mut beams: HashMap<usize, usize> = HashMap::new();
    beams.insert(lines.next().unwrap().find('S').unwrap(), 1);

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '^' {
                if let Some(&count) = beams.get(&i) {
                    if i > 0 {
                        *beams.entry(i - 1).or_insert(0) += count;
                    }
                    if i < line.len() - 1 {
                        *beams.entry(i + 1).or_insert(0) += count;
                    }
                    beams.remove(&i);
                }
            }
        }
    }
    beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 40)
    }
}
