// https://atcoder.jp/contests/abc060/tasks/abc060_b

pub fn run( a: usize, b: usize, c: usize) -> String {
    if (a..=a*b)
        .step_by(a)
        .any(|num| {
            num % b == c
        }) {
            String::from("YES")
        } else {
            String::from("NO")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("YES"), run(7, 5, 1));
        assert_eq!(String::from("NO"), run(2, 2, 1));
        assert_eq!(String::from("YES"), run(1, 100, 97));
        assert_eq!(String::from("YES"), run(40, 98, 58));
        assert_eq!(String::from("NO"), run(77, 42, 36));
    }
}
