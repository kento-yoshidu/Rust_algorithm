// https://atcoder.jp/contests/abc016/tasks/abc016_1

pub fn run(m: usize, d: usize) -> String {
    if m % d == 0 {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(1, 1));
        assert_eq!(String::from("No"), run(2, 29));
        assert_eq!(String::from("Yes"), run(12, 6));
    }
}
