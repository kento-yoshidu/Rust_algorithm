// https://atcoder.jp/contests/abc269/tasks/abc269_a

pub fn run(a: isize, b: isize, c: isize, d: isize) -> (isize, String) {
    ((a + b) * (c - d), String::from("Takahashi"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((6, String::from("Takahashi")), run(1, 2, 5, 3));
        assert_eq!((-700, String::from("Takahashi")), run(10, -20, 30, -40));
        assert_eq!((40000, String::from("Takahashi")), run(100, 100, 100, -100));
    }
}
