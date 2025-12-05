use std::{collections::HashMap, ops::RangeInclusive};

use divisors::get_divisors;

fn main() {
    println!("{}", run("inputs/real.txt"));
}

fn run(path: &str) -> usize {
    let mut out = 0;
    for range in std::fs::read_to_string(path)
        .expect("path must exist")
        .split(',')
    {
        let mut count = 0;
        let mut sums = HashMap::new();
        let (lower, upper) = range.split_once('-').unwrap();
        let (l_digits, u_digits) = (lower.len(), upper.len());
        let digit_range = l_digits..=u_digits;

        for i in 1..=u_digits / 2 {
            for j in divisable(digit_range.clone(), i as usize) {
                let distance = distance(i as u32, j as u32);
                let (a, b) = bounds(lower, upper, i as usize, j, distance);
                count += calc(a, b, distance, i, j, &mut sums);
            }
        }
        out += count;
    }
    out
}

fn bounds(lower: &str, upper: &str, exp: usize, digits: usize, distance: usize) -> (usize, usize) {
    let (l_digits, u_digits) = (lower.len(), upper.len());
    let lower_num: usize = lower.parse().unwrap();
    let upper_num: usize = upper.parse().unwrap();

    let a = if digits == l_digits {
        let lhs = &lower[0..exp].parse::<i32>().unwrap();
        let mut a = *lhs as usize;
        if a * distance >= lower_num {
            a -= 1;
        }
        a
    } else {
        10_usize.pow(exp as u32 - 1) - 1
    };

    let b = if digits == u_digits {
        let lhs = &upper[0..exp].parse::<i32>().unwrap();
        let mut b = *lhs as usize;
        if b * distance > upper_num {
            b -= 1;
        }
        b
    } else {
        10_usize.pow(exp as u32) - 1
    };
    (a, b)
}

fn distance(exp: u32, digits: u32) -> usize {
    let mut distance = 1;
    let mut n_exp = exp;
    while n_exp < digits {
        distance += 10_usize.pow(n_exp);
        n_exp += exp;
    }
    distance
}

fn divisable(range: RangeInclusive<usize>, n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for r in range {
        if r % n == 0 && r >= 2 * n {
            v.push(r);
        }
    }
    v
}
fn calc(
    lower: usize,
    upper: usize,
    distance: usize,
    exp: usize,
    digits: usize,
    sums: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let mut sum = ((upper * (upper + 1) - lower * (lower + 1)) / 2) * distance;
    if sum == 0 {
        return 0;
    }

    for i in get_divisors(exp) {
        sum -= sums.get(&(digits, i)).unwrap_or(&0);
    }

    if exp != 1 {
        sum -= sums.get(&(digits, 1)).unwrap_or(&0);
    }

    sums.insert((digits, exp), sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 4174379265)
    }
}
