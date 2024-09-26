// https://atcoder.jp/contests/abc206/tasks/abc206_a

pub fn run(a: usize, b: usize) -> String {
    if a * 6 >= b && a != 0 {
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
        assert_eq!(String::from("Yes"), run(2, 11));
        assert_eq!(String::from("No"), run(2, 13));
        assert_eq!(String::from("Yes"), run(100, 600));
    }
}
