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
    let symbols = lines
        .next_back()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if !c.is_whitespace() {
                Some((Operation::try_from(c).unwrap(), i))
            } else {
                None
            }
        })
        .collect::<Vec<(Operation, usize)>>();
    let mut problems = vec![Vec::new(); symbols.len()];

    for line in lines {
        for (i, (_, index)) in symbols.iter().enumerate() {
            let v = &mut problems[i];
            let slice = if i < symbols.len() - 1 {
                &line[*index..symbols[i + 1].1 - 1]
            } else {
                &line[*index..]
            };
            for (j, c) in slice.chars().enumerate() {
                if let Some(d) = c.to_digit(10).map(|d| d as usize) {
                    if let Some(num) = v.get_mut(j) {
                        *num = (*num) * 10 + d;
                    } else {
                        v.push(d);
                    }
                } else if v.get(j).is_none() {
                    v.push(0);
                }
            }
        }
    }

    problems.into_iter().enumerate().fold(0, |acc, (i, v)| {
        v.into_iter()
            .reduce(|sum, d| match symbols[i].0 {
                Operation::Add => sum + d,
                Operation::Multiply => sum * d,
            })
            .unwrap()
            + acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 3263827)
    }
}
