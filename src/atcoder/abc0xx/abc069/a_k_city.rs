// https://atcoder.jp/contests/abc069/tasks/abc069_a

pub fn run(n: i32, m: i32) -> i32 {
    (n - 1) * (m - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(3, 4));
        assert_eq!(1, run(2, 2));
    }
}
