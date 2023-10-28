// https://atcoder.jp/contests/abc041/tasks/abc041_b

pub fn run(a: usize, b: usize, c: usize) -> usize {
    (a * b) % 1000000007 * c % 1000000007
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(24, run(2, 3, 4));
        assert_eq!(1000000000, run(10000, 1000, 100));
        assert_eq!(999999937, run(100000, 1, 100000));
        assert_eq!(999999664, run(1000000000, 1000000000, 1000000000));
    }
}
