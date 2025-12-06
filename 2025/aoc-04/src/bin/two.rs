use std::collections::HashMap;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let file = std::fs::read_to_string(path).expect("path must exist");
    let mut neighbours = vec![vec![0; file.find('\n').unwrap()]];
    let mut rolls = HashMap::new();
    let y_max = neighbours[0].len() - 1;
    for (x, line) in file.lines().enumerate() {
        let mut row = vec![0; y_max + 1];
        for (y, c) in line.chars().enumerate() {
            if c == '@' {
                if x > 0 {
                    neighbours[x - 1][y] += 1;
                    if y > 0 {
                        neighbours[x - 1][y - 1] += 1;
                    }
                    if y < y_max {
                        neighbours[x - 1][y + 1] += 1;
                    }
                }
                if y < y_max {
                    neighbours[x][y + 1] += 1;
                    row[y + 1] += 1;
                }
                if y > 0 {
                    neighbours[x][y - 1] += 1;
                    row[y - 1] += 1;
                }
                row[y] += 1;
                rolls.insert((x, y), 0xb);
            }
        }

        neighbours.push(row);
    }
    let mut changed = true;
    let mut total = 0;
    while changed {
        changed = false;
        total += neighbours
            .clone()
            .into_iter()
            .enumerate()
            .fold(0, |acc, (x, v)| {
                acc + v.into_iter().enumerate().fold(0, |acc2, (y, num)| {
                    if rolls.contains_key(&(x, y)) && num < 4 {
                        changed = true;
                        rolls.remove(&(x, y));
                        if x > 0 {
                            neighbours[x - 1][y] -= 1;
                            if y > 0 {
                                neighbours[x - 1][y - 1] -= 1;
                            }
                            if y < y_max {
                                neighbours[x - 1][y + 1] -= 1;
                            }
                        }
                        if y < y_max {
                            neighbours[x][y + 1] -= 1;
                            neighbours[x + 1][y + 1] -= 1;
                        }
                        if y > 0 {
                            neighbours[x][y - 1] -= 1;
                            neighbours[x + 1][y - 1] -= 1;
                        }
                        neighbours[x + 1][y] -= 1;
                        acc2 + 1
                    } else {
                        acc2
                    }
                })
            });
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 43)
    }
}
