// https://atcoder.jp/contests/abc113/tasks/abc113_a

fn run(x: i32, y: i32) -> i32 {
    x + y / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(110, run(81, 58));
        assert_eq!(31, run(4, 54));
    }
}
