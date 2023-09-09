// https://atcoder.jp/contests/abc240/tasks/abc240_a

pub fn run(a: i8, b: i8) -> String {
    if (a - b).abs() == 1 || (a - b).abs() == 9 {
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
        assert_eq!(String::from("Yes"), run(2, 3));
        assert_eq!(String::from("No"), run(3, 5));
        assert_eq!(String::from("Yes"), run(1, 10));
        assert_eq!(String::from("Yes"), run(1, 2));
        assert_eq!(String::from("Yes"), run(2, 3));
        assert_eq!(String::from("Yes"), run(3, 4));
        assert_eq!(String::from("Yes"), run(4, 5));
        assert_eq!(String::from("Yes"), run(5, 6));
        assert_eq!(String::from("No"), run(1, 5));
        assert_eq!(String::from("No"), run(3, 9));
    }
}
