/// 判断是否为 质数
pub fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

#[derive(Debug)]
struct Test {
    r#in: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

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

    #[test]
    fn test_mut() {
        // &mut &[T] 的意思
        // let value = 1;
        // let mut shared: &u32 = &value;
        // println!("{r:p}: {r} (value = {v})", r = shared, v = value);

        // let unique: &mut &u32 = &mut shared;
        // *unique = &17;
        // println!("{r:p}: {r} (value = {v})", r = shared, v = value);

        // let mut slice: &[u8] = &[0, 1, 2, 3, 4];
        // let umique = &mut slice;
        // println!("{r:p}: {len}: {r:?}", r = *umique, len = umique.len());

        // *umique = &umique[..4];
        // println!("{r:p}: {len}: {r:?}", r = *umique, len = umique.len());

        // *umique = &umique[1..];
        // println!("{r:p}: {len}: {r:?}", r = *umique, len = umique.len());

        // *umique = &[17, 17, 42];
        // println!("{r:p}: {len}: {r:?}", r = *umique, len = umique.len());

        // println!("{r:p}: {len}: {r:?}", r = slice, len = slice.len());

        let mut data: &[u8] = &[0, 1, 2, 3, 4, 5, 6];
        let mut buf = [0; 3];

        while let Ok(1..) = Read::read(&mut data, &mut buf) {
            println!("{r:p}: {len}: {r:?}", r = data, len = data.len());
        }

        let test = Test {
            r#in: "sadf".to_string(),
        };
        println!("{:?}", test);
    }
}
