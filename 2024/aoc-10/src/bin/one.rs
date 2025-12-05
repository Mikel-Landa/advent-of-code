use std::collections::HashMap;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}
fn run(path: &str) -> usize {
    let mut map = Vec::new();
    let mut routes: HashMap<(usize, usize), usize> = HashMap::new();
    let mut zeros = Vec::new();
    for (i, line) in std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
        .enumerate()
    {
        let row = line
            .chars()
            .flat_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        row.iter().enumerate().for_each(|(j, e)| {
            if *e == 0 {
                zeros.push((i, j));
            }
        });
        map.push(row);
    }
    for zero in &zeros {
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let r = find_routes(*zero, &map, &routes, direction, Vec::new());
            // println!("{:?}", r);
            // println!("{}", r.len());
            r.iter().for_each(|p| {
                routes.entry(*p).and_modify(|e| *e += 1).or_insert(1);
            });
        }
    }
    zeros.iter().fold(0, |acc, p| {
        let a = routes.get(p).unwrap_or(&0);
        acc + a
    })
}

fn find_routes(
    current: (usize, usize),
    map: &Vec<Vec<u32>>,
    routes: &HashMap<(usize, usize), usize>,
    direction: Direction,
    mut path: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let v = direction.vector();
    let new = (current.0 as isize + v.0, current.1 as isize + v.1);

    if new.0 < 0
        || new.1 < 0
        || new.0 >= map.len() as isize
        || new.1 >= map[current.0].len() as isize
    {
        return Vec::new();
    }
    let new = (new.0 as usize, new.1 as usize);

    if map[new.0][new.1] != map[current.0][current.1] + 1 {
        return Vec::new();
    }

    path.push(current);
    if map[new.0][new.1] == 9 {
        path.push(new);
        return Vec::new();
    }
    // if routes.contains_key(&new) {
    //     out.push(new);
    //     return out;
    // }
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        path.extend(find_routes(new, map, routes, direction, path.clone()));
    }
    println!("{:?}", path);
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 36)
    }
}
