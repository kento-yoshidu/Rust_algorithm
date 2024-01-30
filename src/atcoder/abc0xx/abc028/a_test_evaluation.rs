// https://atcoder.jp/contests/abc028/tasks/abc028_a

pub fn run(n: usize) -> String {
    if n == 100 {
        String::from("Perfect")
    } else if n >= 90 {
        String::from("Great")
    } else if n >= 60 {
        String::from("Good")
    } else {
        String::from("Bad")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Good"), run(80));
        assert_eq!(String::from("Perfect"), run(100));
        assert_eq!(String::from("Bad"), run(59));
        assert_eq!(String::from("Great"), run(95));
    }
}
