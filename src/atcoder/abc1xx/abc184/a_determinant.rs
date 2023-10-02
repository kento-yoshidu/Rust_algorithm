// https://atcoder.jp/contests/abc184/tasks/abc184_a

pub fn run(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a * d - b * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(-2, run(1, 2, 3, 4));
        assert_eq!(1, run(0, -1, 1, 0));
        assert_eq!(0, run(100, 100, 100, 100));
    }
}
