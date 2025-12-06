fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> u32 {
    let mut sum = 0;
    for line in std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
    {
        sum += max_joltage(line);
    }
    sum
}

fn max_joltage(bank: &str) -> u32 {
    let mut v = bank.chars().map(|c| c.to_digit(10).unwrap()).peekable();
    let mut a = v.next().unwrap();
    let mut b = v.next().unwrap();
    let mut replaced = false;
    while let Some(x) = v.next() {
        if x > a && v.peek().is_some() {
            a = x;
            replaced = true;
            continue;
        }
        if x > b || replaced {
            b = x;
            replaced = false;
        }
    }
    a * 10 + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 357)
    }
}
