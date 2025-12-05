use std::collections::HashSet;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

type IndexDirection = isize;
type MatchDirection = isize;
type MatchIndex = usize;

fn match_letter(
    c: char,
    i: usize,
    lines: &Vec<&str>,
    count: &mut HashSet<Vec<(usize, usize)>>,
    mut n: Vec<(usize, usize)>,
    mut pos: (MatchIndex, MatchDirection, IndexDirection),
) {
    let i = i as isize + pos.2;
    let match_index = n.last().unwrap().0 as isize + pos.1;
    if i < 0 || match_index < 0 {
        return;
    }

    let i = i as usize;
    let match_index = match_index as usize;
    if i >= lines.len() || match_index >= lines[i].len() {
        return;
    }

    let found = lines[i].chars().nth(match_index).unwrap() == c;
    if !found {
        return;
    } else if c == 'S' {
        n.push((match_index, i));
        count.insert(n);
        return;
    }

    let nc = match c {
        'M' => 'A',
        'A' => 'S',
        _ => return,
    };

    let mut n = n.clone();
    n.push((match_index, i));
    pos.0 = match_index;

    match_letter(nc, i, lines, count, n.clone(), pos);
}

fn run(path: &str) -> usize {
    let lines = std::fs::read_to_string(path).expect("must provide a valid file path");
    let lines: Vec<_> = lines.lines().collect();
    let mut count = HashSet::new();
    for i in 0..lines.len() {
        for (m, _) in lines[i].match_indices('X') {
            let nc = 'M';
            let n = vec![(m, i)];
            let direction = [-1, 0, 1];
            for index_direction in direction {
                for match_direction in direction {
                    match_letter(
                        nc,
                        i,
                        &lines,
                        &mut count,
                        n.clone(),
                        (m, match_direction, index_direction),
                    );
                }
            }
        }
    }
    count.len()
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 18);
    }
}
