use itertools::peek_nth;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let mut sum = 0;
    for line in std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
    {
        sum += max_joltage(line);
    }
    sum
}

fn max_joltage(bank: &str) -> usize {
    let mut v = peek_nth(bank.chars().map(|c| c.to_digit(10).unwrap()));
    let mut out = [0; 12];
    let mut replaced = [false; 12];
    while let Some(x) = v.next() {
        for (i, num) in out.iter_mut().enumerate() {
            if (x > *num && (i == 11 || v.peek_nth(10 - i).is_some())) || replaced[i] {
                *num = x;
                replaced[i] = false;
                replaced[i + 1..].fill(true);
                break;
            }
        }
    }
    out.iter().enumerate().fold(0_usize, |acc, (i, num)| {
        10_usize.pow(11 - i as u32) * (*num as usize) + acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 3121910778619)
    }
}
