fn main() {
    println!("{}", run("inputs/real.txt"));
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Operation::Multiply),
            '+' => Ok(Operation::Add),
            _ => Err("must match * or +".to_string()),
        }
    }
}

fn run(path: &str) -> usize {
    let file = std::fs::read_to_string(path).expect("path must exist");
    let mut lines = file.lines();
    let mut problems = Vec::new();
    let symbols = lines
        .next_back()
        .unwrap()
        .chars()
        .filter_map(|c| {
            if !c.is_whitespace() {
                let o = Operation::try_from(c).unwrap();
                match o {
                    Operation::Add => problems.push(0),
                    Operation::Multiply => problems.push(1),
                }
                Some(o)
            } else {
                None
            }
        })
        .collect::<Vec<Operation>>();
    for line in lines {
        let mut i = 0;
        for s in line.split(" ") {
            if s.is_empty() {
                continue;
            }
            let n: usize = s.parse().unwrap();
            match symbols[i] {
                Operation::Add => problems[i] += n,
                Operation::Multiply => problems[i] *= n,
            }
            i += 1;
        }
    }

    problems.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 4277556)
    }
}
