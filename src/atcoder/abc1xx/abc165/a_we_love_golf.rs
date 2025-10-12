// https://atcoder.jp/contests/abc165/tasks/abc165_a

fn run(k: usize, a: usize, b: usize) -> &'static str {
    for n in a..=b {
        if n % k == 0 {
            return "OK";
        }
    }

    "NG"
}

fn run2(k: usize, a: usize, b: usize) -> &'static str {
    if (a..=b).any(|num| {
        num % k == 0
    }) {
        "OK"
    } else {
        "NG"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc165_a() {
        let tests = [
            TestCase(7, 500, 600, "OK"),
            TestCase(4, 5, 7, "NG"),
            TestCase(1, 11, 11, "OK"),
        ];

        for TestCase(k, a, b, expected) in tests {
            assert_eq!(run(k, a, b), expected);
        }
    }
}
