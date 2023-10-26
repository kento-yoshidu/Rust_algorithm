// https://atcoder.jp/contests/abc013/tasks/abc013_2

pub fn run(a: i32, b: i32) -> i32 {
    ((b - a).abs()).min(10 - a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, 6));
        assert_eq!(2, run(6, 4));
        assert_eq!(3, run(8, 1));
    }
}
