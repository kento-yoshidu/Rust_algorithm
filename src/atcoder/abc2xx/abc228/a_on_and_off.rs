// https://atcoder.jp/contests/abc228/tasks/abc228_a

pub fn run(s: usize, t: usize, x: usize) -> String {
    if (s..)
        .take_while(|time| time % 24 != t)
        .any(|time| time % 24 == x)
    {
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
        assert_eq!(String::from("Yes"), run(7, 20, 12));
        assert_eq!(String::from("No"), run(20, 7, 12));
        assert_eq!(String::from("Yes"), run(23, 0, 23));
    }
}
