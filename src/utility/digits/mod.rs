pub type Split = Vec<u8>;

/// Joins a Split into its corresponding number.
pub fn join(split: &Split) -> u64 {
    let mut number: u64 = 0;
    for (power_of_ten, digit) in split.iter().rev().enumerate() {
        number += (*digit as u64) * 10_u64.pow(power_of_ten as u32);
    }
    number
}

/// Splits a number into a vector of digits.
pub fn split(number: u64) -> Split {
    let number_string = number.to_string();
    let mut digit_vec = Vec::with_capacity(number_string.len());
    digit_vec.extend(number_string.chars().map(|c| c.to_digit(10).unwrap() as u8));
    digit_vec
}

#[cfg(test)]
mod tests {
    use super::join;
    use super::split;

    #[test]
    fn test_join() {
        assert_eq!(join(&vec![1]), 1);
        assert_eq!(join(&vec![1, 2, 3, 4]), 1234);
        assert_eq!(join(&vec![0, 1, 2, 3, 4]), 1234);
        assert_eq!(join(&vec![1, 2, 0, 3, 4]), 12034);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(1), vec![1]);
        assert_eq!(split(1234), vec![1, 2, 3, 4]);
        assert_eq!(split(12034), vec![1, 2, 0, 3, 4]);
        assert_ne!(split(1234), vec![1, 2, 3, 4, 5]);
        assert_ne!(split(12345), vec![1, 2, 3, 4]);
    }
}