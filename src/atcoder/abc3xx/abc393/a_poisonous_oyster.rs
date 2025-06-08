// https://atcoder.jp/contests/abc393/tasks/abc393_a

fn run(s1: &str, s2: &str) -> usize {
    if s1 == "sick" && s2 == "sick" {
        1
    } else if s1 == "sick" && s2 == "fine" {
        2
    } else if s1 == "fine" && s2 == "sick" {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("sick", "fine", 2),
            TestCase("fine", "fine", 4),
        ];

        for TestCase(s1, s2, expected) in tests {
            assert_eq!(run(s1, s2), expected);
        }
    }
}
