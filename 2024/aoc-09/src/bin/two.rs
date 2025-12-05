fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let disk_map: Vec<usize> = std::fs::read_to_string(path)
        .expect("path must exist")
        .split("")
        .flat_map(|s| s.parse())
        .collect();
    convert(&disk_map)
}

fn convert(disk_map: &Vec<usize>) -> usize {
    let mut indexes = vec![0, disk_map[0]];
    disk_map.iter().cloned().reduce(|acc, e| {
        let a = acc + e;
        indexes.push(a);
        a
    });
    let mut records = disk_map.clone();
    let last = (disk_map.len() - 1) / 2 * 2;
    let mut out: usize = 0;
    for file_p in (0..=last).step_by(2) {
        let file_p = last - file_p;
        let mut free_p = 1;
        let mut changed = false;
        while free_p < file_p {
            if records[free_p] >= disk_map[file_p] {
                let index = indexes[free_p] + disk_map[free_p] - records[free_p];
                out += (index..index + disk_map[file_p]).sum::<usize>() * (file_p / 2);
                records[free_p] -= records[file_p];
                changed = true;
                break;
            }
            free_p += 2;
        }
        if !changed {
            let index = indexes[file_p];
            out += (index..index + disk_map[file_p]).sum::<usize>() * (file_p / 2);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 2858)
    }
}
