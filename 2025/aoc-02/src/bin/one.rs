fn main() {
    println!("{}", run("inputs/real.txt"));
}

/// Main function that processes all ID ranges and sums up invalid IDs.
///
/// Part 1: An invalid ID is a number made of a digit pattern repeated exactly twice.
/// Examples: 11 (pattern "1"×2), 1212 (pattern "12"×2), 123123 (pattern "123"×2)
///
/// Algorithm approach:
/// 1. Count all numbers with even digit counts that could be repeated patterns
/// 2. Subtract numbers below the lower bound (inclusion-exclusion)
/// 3. Subtract numbers above the upper bound (inclusion-exclusion)
fn run(path: &str) -> usize {
    let mut count = 0;
    // Process each range separately (ranges are comma-separated)
    for range in std::fs::read_to_string(path)
        .expect("path must exist")
        .split(',')
    {
        let (lower, upper) = range.split_once('-').unwrap();
        let (l_digits, u_digits) = (lower.len(), upper.len());

        // Count all numbers with even digit counts in the range [l_digits, u_digits]
        // For a number with 'digits' digits, if digits is even, it could be a pattern
        // of length exp = digits/2 repeated twice
        for i in 0..=u_digits - l_digits {
            let digits = l_digits + i;
            if digits % 2 == 0 {
                // Pattern length is half the total digits
                let exp = digits as u32 / 2;
                // Count all numbers with this digit count
                // a = maximum pattern value (10^exp - 1)
                // b = minimum pattern value (10^(exp-1) - 1)
                // We'll count from b+1 to a, which gives all valid patterns
                let a = 10_usize.pow(exp) - 1;
                let b = 10_usize.pow(exp - 1) - 1;
                count += calc(exp, b, a);
            }
        }

        // Subtract numbers below the lower bound using inclusion-exclusion
        // If the lower bound has an even number of digits, we've overcounted
        // numbers that are less than the lower bound
        if l_digits % 2 == 0 {
            let exp = l_digits / 2;
            // Split lower bound into two halves: lhs (first exp digits) and rhs (last exp digits)
            // For a repeated pattern number, lhs should equal rhs
            let (lhs, rhs) = (
                &lower[0..exp].parse::<i32>().unwrap(),
                &lower[exp..].parse::<i32>().unwrap(),
            );
            let exp = exp as u32;

            // Find the maximum pattern value that's still < lower bound
            let mut a = *lhs as usize;
            // If rhs <= lhs, then lhs*pattern gives a number >= lower, so we need to go back one
            if rhs <= lhs {
                a -= 1;
            }
            // Subtract all numbers from minimum to 'a' (these are < lower bound)
            count -= calc(exp, 10_usize.pow(exp - 1) - 1, a);
        }

        // Subtract numbers above the upper bound using inclusion-exclusion
        // If the upper bound has an even number of digits, we've overcounted
        // numbers that are greater than the upper bound
        if u_digits % 2 == 0 {
            let exp = u_digits / 2;
            // Split upper bound into two halves: lhs (first exp digits) and rhs (last exp digits)
            let (lhs, rhs) = (
                &upper[0..exp].parse::<i32>().unwrap(),
                &upper[exp..].parse::<i32>().unwrap(),
            );
            let exp = exp as u32;

            // Find the minimum pattern value that's still > upper bound
            let mut b = *lhs as usize;
            // If rhs < lhs, then lhs*pattern gives a number > upper, so we need to go back one
            if rhs < lhs {
                b -= 1;
            }
            // Subtract all numbers from 'b+1' to maximum (these are > upper bound)
            count -= calc(exp, b, 10_usize.pow(exp) - 1);
        }
    }
    count
}

/// Calculate the sum of all numbers matching a repeated pattern.
///
/// For Part 1, a number is made of a pattern repeated exactly twice.
/// If the pattern has length `exp`, the number can be written as: pattern_value * distance
/// where distance = 10^exp + 1
///
/// Example: pattern "12" (exp=2) gives number 1212
/// - 1212 = 12 * (10^2 + 1) = 12 * 101
/// - distance = 10^2 + 1 = 101
///
/// Parameters:
/// - exp: length of the repeated pattern
/// - lower, upper: bounds for the pattern value (we count from lower+1 to upper)
///
/// Returns: sum of all numbers pattern_value * distance for pattern_value in [lower+1, upper]
fn calc(exp: u32, lower: usize, upper: usize) -> usize {
    // Distance multiplier: for a pattern of length exp repeated twice,
    // distance = 10^exp + 1
    // This converts the pattern value to the full number
    let distance = 10_usize.pow(exp) + 1;
    // Sum of pattern values from (lower+1) to upper
    // Formula: sum(x) for x in [lower+1, upper] = (upper*(upper+1) - lower*(lower+1)) / 2
    let sum = (upper * (upper + 1) - lower * (lower + 1)) / 2;
    // Multiply by distance to get sum of actual numbers
    sum * distance
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(run("inputs/example.txt"), 1227775554)
    }
}
