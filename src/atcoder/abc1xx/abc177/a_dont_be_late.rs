// https://atcoder.jp/contests/abc177/tasks/abc177_a

pub fn run(d: usize, t: usize, s: usize) -> String {
    if d <= s * t {
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
        assert_eq!(String::from("Yes"), run(1000, 15, 80));
        assert_eq!(String::from("Yes"), run(2000, 20, 100));
        assert_eq!(String::from("No"), run(10000, 1, 1));
    }
}
