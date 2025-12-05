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
                        let mut overflow = (false, false);
                        let (pos_x, pos_y) = (freq.0 - x, freq.1 - y);
                        let mut i = 0;
                        while !overflow.0 || !overflow.1 {
                            let up = (freq.0 + pos_x * i, freq.1 + pos_y * i);
                            let down = (x - pos_x * i, y - pos_y * i);
                            if up.0 >= 0 && up.0 < len_x && up.1 >= 0 && up.1 < len_y {
                                sum.insert(up);
                            } else {
                                overflow.0 = true;
                            }
                            if down.0 >= 0 && down.0 < len_x && down.1 >= 0 && down.1 < len_y {
                                sum.insert(down);
                            } else {
                                overflow.1 = true;
                            }
                            i += 1
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
        assert_eq!(run("inputs/example.txt"), 34)
    }
}
