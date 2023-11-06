// https://atcoder.jp/contests/abc144/tasks/abc144_b

pub fn run (n: usize) -> String {
    if n > 81 {
        return String::from("No");
    }

    for i in 0..=9 {
        for j in 0..=9 {
            if i * j == n {
                return String::from("Yes");
            }
        }
    }

    String::from("No")
}

pub fn run2(n: usize) -> String {
    use itertools::Itertools;

    if (1..=9).combinations_with_replacement(2)
        .any(|t| {
            t[0] * t[1] == n
        }) {
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
        assert_eq!(String::from("Yes"), run(10));
        assert_eq!(String::from("No"), run(50));
        assert_eq!(String::from("Yes"), run(81));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(10));
        assert_eq!(String::from("No"), run2(50));
        assert_eq!(String::from("Yes"), run2(81));
    }
}
