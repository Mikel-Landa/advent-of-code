use std::collections::HashSet;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

type Vector = Point;

impl Vector {
    fn rotate(&mut self) {
        let x = self.x;
        // because we invert directions due to indexes, formula needs to be inverted as well
        self.x = -self.y;
        self.y = x;
    }
}

impl Point {
    fn add_vector(&self, v: &Vector) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}
fn traverse(
    mut position: Point,
    direction: &mut Vector,
    lines: &Vec<Vec<char>>,
    traversed: &mut HashSet<(Point, Vector)>,
) -> bool {
    let new_pos = position.add_vector(direction);
    if new_pos.x < 0
        || new_pos.y < 0
        || new_pos.y as usize >= lines.len()
        || new_pos.x as usize >= lines[new_pos.y as usize].len()
    {
        return false;
    }

    match lines[new_pos.y as usize][new_pos.x as usize] {
        '.' => {
            position = new_pos;
            if !traversed.insert((position.clone(), direction.clone())) {
                return true;
            }
        }
        '#' => {
            direction.rotate();
        }
        c => panic!("{c} was an unexpected character"),
    }
    traverse(position, direction, lines, traversed)
}

fn run(path: &str) -> usize {
    let mut lines: Vec<Vec<char>> = std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut position = Point { x: 0, y: 0 };
    let mut direction = Vector { x: 0, y: 0 };
    let mut traversed = HashSet::new();
    'outer: for (y, line) in lines.iter_mut().enumerate() {
        for (x, char) in line.iter_mut().enumerate() {
            match char {
                '>' => {
                    direction.x = 1;
                }
                '<' => {
                    direction.x = -1;
                }
                '^' => {
                    direction.y = -1;
                }
                'v' => {
                    direction.y = 1;
                }
                _ => continue,
            }
            position.x = x as isize;
            position.y = y as isize;
            traversed.insert((position.clone(), direction.clone()));
            *char = '.';
            break 'outer;
        }
    }
    let position_loop = position.clone();
    let direction_loop = direction.clone();
    traverse(position, &mut direction, &lines, &mut traversed);
    let mut loop_traversed = HashSet::from_iter([(position_loop.clone(), direction_loop.clone())]);
    let mut count = 0;
    let traversed = traversed
        .into_iter()
        .map(|(point, _)| point)
        .collect::<HashSet<Point>>();
    for point in traversed {
        lines[point.y as usize][point.x as usize] = '#';
        let position = position_loop.clone();
        let mut direction = direction_loop.clone();
        if traverse(
            position.clone(),
            &mut direction,
            &lines,
            &mut loop_traversed,
        ) {
            count += 1;
        }
        loop_traversed = HashSet::from_iter([(position_loop.clone(), direction_loop.clone())]);
        lines[point.y as usize][point.x as usize] = '.';
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 6);
    }
}
