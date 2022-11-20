/// 判断是否为 质数
pub fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_hundred_primes() {
        let primes: Vec<u32> = (2..100).filter(|n| is_prime(*n)).collect();
        assert_eq!(
            primes,
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        )
    }
}
