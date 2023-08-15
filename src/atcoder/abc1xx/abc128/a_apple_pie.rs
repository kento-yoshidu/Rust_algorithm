// https://atcoder.jp/contests/abc128/tasks/abc128_a

pub fn run(a: usize, p: usize) -> usize {
    let piece = p + a * 3;

    piece / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(1, 3));
        assert_eq!(0, run(0, 1));
        assert_eq!(58, run(32, 21));
    }
}
