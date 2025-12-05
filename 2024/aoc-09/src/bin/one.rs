fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let disk_map: Vec<usize> = std::fs::read_to_string(path)
        .expect("path must exist")
        .split("")
        .flat_map(|s| s.parse())
        .collect();
    let fi = disk_map[0];
    let mut unwrapped_disk_map = convert(&disk_map[1..]);
    sort(&mut unwrapped_disk_map);
    let mut sum = 0;
    for (index, id) in unwrapped_disk_map.iter().enumerate() {
        sum += (index + fi) * id;
    }
    sum
}

fn convert(disk_map: &[usize]) -> Vec<usize> {
    let mut id = 1;
    let mut out = Vec::new();
    for (i, size) in disk_map.iter().enumerate() {
        if i % 2 != 0 {
            out.extend(vec![id; *size]);
            id += 1;
        } else {
            out.extend(vec![0; *size]);
        }
    }
    out
}
fn sort(unwrapped_disk_map: &mut Vec<usize>) {
    let mut file_p = unwrapped_disk_map.len() - 1;
    let mut free_p = 0;
    while free_p < file_p {
        if unwrapped_disk_map[file_p] == 0 {
            file_p -= 1;
            continue;
        }
        if unwrapped_disk_map[free_p] != 0 {
            free_p += 1;
            continue;
        }
        unwrapped_disk_map[free_p] = unwrapped_disk_map[file_p];
        unwrapped_disk_map[file_p] = 0;
        file_p -= 1;
        free_p += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 1928)
    }
}
