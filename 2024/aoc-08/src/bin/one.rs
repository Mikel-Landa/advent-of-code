use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let mut locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let lines = std::fs::read_to_string(path).expect("path must exist");
    let lines = lines.lines().collect::<Vec<&str>>();
    let len_x = lines.len() as isize;
    let len_y = lines[0].len() as isize;
    let mut sum = HashSet::new();

    for (line, x) in lines.into_iter().zip(0isize..) {
        for (char, y) in line.chars().zip(0isize..) {
            if char != '.' {
                if let Some(freq_list) = locations.get_mut(&char) {
                    for freq in freq_list.iter() {
                        let (pos_x, pos_y) = (freq.0 - x, freq.1 - y);
                        let up = (freq.0 + pos_x, freq.1 + pos_y);
                        let down = (x - pos_x, y - pos_y);
                        if up.0 >= 0 && up.0 < len_x && up.1 >= 0 && up.1 < len_y {
                            sum.insert(up);
                        }
                        if down.0 >= 0 && down.0 < len_x && down.1 >= 0 && down.1 < len_y {
                            sum.insert(down);
                        }
                    }
                    freq_list.push((x, y));
                } else {
                    locations.insert(char, vec![(x, y)]);
                }
            }
        }
    }

    sum.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 14)
    }
}
