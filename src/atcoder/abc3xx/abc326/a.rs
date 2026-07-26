// https://atcoder.jp/contests/abc326/tasks/abc326_a

fn run(x: isize, y: isize) -> &'static str {
    let dis = x - y;

    if dis.abs() > 3 {
        "No"
    } else if dis < 0 && dis.abs() == 3 {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, &'static str);

    #[test]
    fn abc326_a() {
        let tests = [
            TestCase(1, 4, "No"),
            TestCase(99, 96, "Yes"),
            TestCase(100, 1, "No"),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
