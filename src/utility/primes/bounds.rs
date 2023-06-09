/// Upper bound for the number of primes below and including the given number.
/// 
/// This number will always be greater than the number of primes below and
/// including the given number. Source: https://en.wikipedia.org/wiki/Prime-counting_function#Inequalities.
pub fn prime_count_upper_bound(number: u64) -> u64 {
    if number <= 1 {
        return 1;
    }
    let upper_bound = 1.25506 * ((number as f64) / (number as f64).ln());
    upper_bound.ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::prime_count_upper_bound;

    #[test]
    fn test_prime_count_upper_bound() {
        assert_eq!(prime_count_upper_bound(0), 1);
        assert_eq!(prime_count_upper_bound(1), 1);
        assert_eq!(prime_count_upper_bound(2), 4);
        assert_eq!(prime_count_upper_bound(3), 4);
        assert_eq!(prime_count_upper_bound(4), 4);
        assert_eq!(prime_count_upper_bound(5), 4);
        assert_eq!(prime_count_upper_bound(6), 5);
        assert_eq!(prime_count_upper_bound(4567), 681);
        assert_eq!(prime_count_upper_bound(6785678), 541403);
    }
}