// https://atcoder.jp/contests/abc213/tasks/abc213_a

fn run(a: usize, b: usize) -> usize {
    a ^ b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(3, 6));
        assert_eq!(6, run(10, 12));
    }
}
