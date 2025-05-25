// https://atcoder.jp/contests/abc115/tasks/abc115_a

fn run(x: usize) -> &'state str {
    match x {
        25 => "Christmas",
        24 => "Christmas Eve",
        23 => "Christmas Eve Eve",
        _ => "Christmas Eve Eve Eve",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc115_a() {
        let tests = [
            TestCase(25, "Christmas"),
            TestCase(22, "Christmas Eve Eve Eve"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
