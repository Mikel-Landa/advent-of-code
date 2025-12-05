use std::collections::HashSet;
fn main() {
    println!("{}", run("inputs/real.txt"));
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    index: usize,
    position: usize,
}

struct Position {
    match_index: usize,
    match_direction: isize,
    index_direction: isize,
    index: usize,
    char_a: Option<Point>,
}

fn match_letter(
    c: char,
    lines: &Vec<&str>,
    set: &mut HashSet<Point>,
    mut pos: Position,
    count: &mut usize,
) {
    let i = pos.index as isize + pos.index_direction;
    let match_index = pos.match_index as isize + pos.match_direction;
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
        if let Some(pos) = pos.char_a
            && !set.insert(pos)
        {
            *count += 1;
        }
        return;
    } else {
        pos.char_a = Some(Point {
            index: i,
            position: match_index,
        });
    }

    let nc = 'S';

    pos.match_index = match_index;
    pos.index = i;

    match_letter(nc, lines, set, pos, count);
}

fn run(path: &str) -> usize {
    let lines = std::fs::read_to_string(path).expect("must provide a valid file path");
    let lines: Vec<_> = lines.lines().collect();
    let mut set = HashSet::new();
    let mut count = 0;
    for i in 0..lines.len() {
        for (m, _) in lines[i].match_indices('M') {
            let nc = 'A';
            let direction = [-1, 1];
            for index_direction in direction {
                for match_direction in direction {
                    match_letter(
                        nc,
                        &lines,
                        &mut set,
                        Position {
                            match_index: m,
                            match_direction,
                            index_direction,
                            index: i,
                            char_a: None,
                        },
                        &mut count,
                    );
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 9);
    }
}
