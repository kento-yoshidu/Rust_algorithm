// https://atcoder.jp/contests/abc172/tasks/abc172_a

pub fn run(a: usize) -> usize {
    a + a * a + a * a * a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(14, run(2));
        assert_eq!(1110, run(10));
    }
}
