// https://atcoder.jp/contests/abc036/tasks/abc036_a

pub fn run(a: usize, b: usize) -> usize {
    (b as f64 / a as f64).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(12, 36));
        assert_eq!(9, run(12, 100));
    }
}
