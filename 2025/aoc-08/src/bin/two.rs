use std::{collections::HashMap, ops::AddAssign};

fn main() {
    println!("{}", run("inputs/real.txt"));
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn distance(&self, point: &Point) -> f64 {
        let x = self.x.abs_diff(point.x).pow(2);
        let y = self.y.abs_diff(point.y).pow(2);
        let z = self.z.abs_diff(point.z).pow(2);
        ((x + y + z) as f64).sqrt()
    }
}

fn run(path: &str) -> usize {
    let mut points: Vec<Point> = Vec::new();
    let mut distances = Vec::new();
    for line in std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
    {
        let mut coordinates = line.splitn(3, ',');
        let point = Point {
            x: coordinates.next().unwrap().parse().unwrap(),
            y: coordinates.next().unwrap().parse().unwrap(),
            z: coordinates.next().unwrap().parse().unwrap(),
        };
        for p in &points {
            distances.push((point.clone(), p.clone(), point.distance(p)));
        }
        points.push(point);
    }
    distances.sort_by(|(_, _, d1), (_, _, d2)| (*d1).total_cmp(d2));
    let mut pairs: HashMap<Point, (Option<Point>, usize)> = HashMap::new();

    let mut x = 0;
    for (p1, p2, _) in &distances {
        let (pointer, mut c) = update_references(p1.clone(), &mut pairs, None);
        if c < points.len() {
            c = c.max(update_references(p2.clone(), &mut pairs, Some(pointer.clone())).1);
        }
        if c >= points.len() {
            x = p1.x * p2.x;
            break;
        }
    }

    x
}

fn update_references(
    pointer: Point,
    pairs: &mut HashMap<Point, (Option<Point>, usize)>,
    update_ref: Option<Point>,
) -> (Point, usize) {
    let mut point = pointer.clone();

    while let Some((Some(p), _)) = pairs.get(&point).cloned() {
        point = p;
    }
    let len = if let Some(r) = update_ref {
        if r != point {
            let p = pairs.entry(point.clone()).or_insert_with(|| (None, 1));
            p.0 = Some(r.clone());
            pairs.get_mut(&r).unwrap().1 += p.1;
        }
        pairs.get_mut(&r).unwrap().1
    } else if !pairs.contains_key(&pointer) {
        let p = pairs.entry(point.clone()).or_default();
        p.1.add_assign(1);
        p.1
    } else {
        pairs.get(&point).unwrap().1
    };
    (point, len)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 25272)
    }
}
