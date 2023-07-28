// https://atcoder.jp/contests/abc058/tasks/abc058_a

#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> String {
    if b - a == c - b {
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
        assert_eq!(String::from("Yes"), run(2, 4, 6));
        assert_eq!(String::from("No"), run(2, 5, 6));
        assert_eq!(String::from("Yes"), run(3, 2, 1));
    }
}
