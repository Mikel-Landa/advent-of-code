fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> i32 {
    let mut count = 0;
    let mut pointer = 50;
    for line in std::fs::read_to_string(path)
        .expect("path must exist")
        .lines()
    {
        let (direction, rotations) = line.split_at(1);
        let rotations = rotations.parse::<i32>().unwrap();
        let rotations = match direction {
            "L" => -rotations,
            "R" => rotations,
            _ => panic!("Only allow L and R"),
        };
        pointer = (pointer + rotations).rem_euclid(100);
        if pointer == 0 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 3)
    }
}
