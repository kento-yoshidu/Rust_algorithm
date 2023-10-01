// https://atcoder.jp/contests/abc097/tasks/abc097_a

pub fn run(a: isize, b: isize, c: isize, d: isize) -> String {
    if  (a - c).abs() <= d || ((a - b).abs() <= d && (b - c).abs() <= d) {
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
        assert_eq!(String::from("Yes"), run(4, 7, 9, 3));
        assert_eq!(String::from("No"), run(100, 10, 1, 2));
        assert_eq!(String::from("Yes"), run(10, 10, 10, 1));
        assert_eq!(String::from("Yes"), run(1, 100, 2, 10));
    }
}