// https://atcoder.jp/contests/abc471/tasks/abc471_a

fn run(a: isize, b: isize) -> &'static str {
    let mut count = 0;

    if a + b == 9 {
        count += 1;
    }

    if a - b == 9 {
        count += 1;
    }

    if a * b == 9 {
        count += 1;
    }

    if a as f64 / b as f64 == 9.0 {
        count += 1;
    }

    if count >= 1 {
        "Nine"
    } else {
        "Nein"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, &'static str);

    #[test]
    fn abc471_a() {
        let tests = [
            TestCase(16, 7, "Nine"),
            TestCase(66, 7, "Nein"),
            TestCase(9, 1, "Nine"),
            TestCase(9, 9, "Nein"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
