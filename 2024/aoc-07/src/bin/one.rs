fn main() {
    println!("{}", run("inputs/real.txt"));
}

enum Operator {
    Add,
    Multiply,
}

fn run(path: &str) -> usize {
    let mut sum = 0;
    for line in std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
    {
        sum += calibrate(line)
    }
    sum
}

fn calibrate(line: &str) -> usize {
    let mut s = line.split(':');
    let total = s.next().unwrap().parse().unwrap();
    let operands: Vec<_> = s
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    if operate(operands[0], &operands[1..], total, Operator::Add)
        || operate(operands[0], &operands[1..], total, Operator::Multiply)
    {
        total
    } else {
        0
    }
}

fn operate(mut a: usize, b: &[usize], total: usize, operator: Operator) -> bool {
    match operator {
        Operator::Add => a += b[0],
        Operator::Multiply => a *= b[0],
    }
    if a == total && b.len() == 1 {
        return true;
    } else if a > total || b.len() == 1 {
        return false;
    }
    operate(a, &b[1..], total, Operator::Add) || operate(a, &b[1..], total, Operator::Multiply)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 3749)
    }
}
