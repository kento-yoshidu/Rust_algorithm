// https://atcoder.jp/contests/abc182/tasks/abc182_a

pub fn run(a: i32, b: i32) -> i32 {
    (a * 2 + 100) - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(200, run(200, 300));
        assert_eq!(20100, run(10000, 0));
    }
}
