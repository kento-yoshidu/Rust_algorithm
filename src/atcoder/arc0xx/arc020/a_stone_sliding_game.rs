// https://atcoder.jp/contests/arc020/tasks/arc020_1

fn run(a: isize, b: isize) -> &'static str {
    if a.abs() < b.abs() {
        "Ant"
    } else if a.abs() == b.abs() {
        "Draw"
    } else {
        "Bug"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, "Ant"),
            TestCase(1, 0, "Bug"),
            TestCase(-100, 100, "Draw"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
