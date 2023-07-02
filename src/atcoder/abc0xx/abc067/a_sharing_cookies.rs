// https://atcoder.jp/contests/abc067/tasks/abc067_a

#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> String {
    if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
        String::from("Possible")
    } else {
        String::from("Impossible")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Possible"), run(4, 5));
        assert_eq!(String::from("Impossible"), run(1, 1));
    }
}
