// https://atcoder.jp/contests/abc192/tasks/abc192_a

pub fn run(x: i32) -> i32 {
    if x % 100 == 0 {
        100
    } else {
        100 - x % 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(60, run(140));
        assert_eq!(100, run(1000));
    }
}
