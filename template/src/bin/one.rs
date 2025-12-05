fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    for line in std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
    {}
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 0)
    }
}
