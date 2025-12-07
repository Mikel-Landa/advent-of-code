use std::collections::HashSet;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let file = std::fs::read_to_string(path).expect("path must exist");
    let mut lines = file.lines();
    let mut beams = HashSet::new();
    beams.insert(lines.next().unwrap().find('S').unwrap());
    let mut count = 0;
    for line in lines {
        let mut n_beams = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            if c == '^' && beams.contains(&i) {
                count += 1;
                if i > 0 {
                    n_beams.insert(i - 1);
                }
                if i < line.len() - 1 {
                    n_beams.insert(i + 1);
                }
                beams.remove(&i);
            }
        }
        beams.extend(n_beams);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 21)
    }
}
