// https://atcoder.jp/contests/abc267/tasks/abc267_a

fn run(s: &str) -> usize {
    let arr = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    arr.into_iter().rev().position(|day| day == s).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc267_a() {
        let tests = [
            TestCase("Friday", 1),
            TestCase("Thursday", 2),
            TestCase("Wednesday", 3),
            TestCase("Tuesday", 4),
            TestCase("Monday", 5),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
