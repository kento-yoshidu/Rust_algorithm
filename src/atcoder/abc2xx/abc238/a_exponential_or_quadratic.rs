// https://atcoder.jp/contests/abc238/tasks/abc238_a

pub fn run(n: usize) -> String {
    if 2 <= n && n <= 4 {
        String::from("No")
    } else {
        String::from("Yes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5));
        assert_eq!(String::from("No"), run(2));
        assert_eq!(String::from("Yes"), run(623947744));
    }
}

