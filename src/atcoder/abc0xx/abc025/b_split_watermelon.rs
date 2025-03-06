// https://atcoder.jp/contests/abc025/tasks/abc025_b

fn run(_n: usize, a: isize, b: isize, v: Vec<(&str, isize)>) -> (&'static str, isize) {
    let point: isize = v.iter()
        .map(|t| {
            let distance = t.1.max(a).min(b);

            if t.0 == "East" {
                distance
            } else {
                -distance
            }
        })
        .sum();

    if point < 0 {
        ("West", -point)
    } else if 0 < point {
        ("East", point)
    } else {
        ("0", 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, isize, Vec<(&'static str, isize)>, (&'static str, isize));

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, 10, vec![("East", 7), ("West", 3), ("West", 11)], ("West", 8)),
            TestCase(3, 3, 8, vec![("West", 6), ("East", 3), ("East", 1)], ("0", 0)),
            TestCase(3, 25, 25, vec![("East", 1), ("East", 1), ("West", 1), ("East", 100), ("West", 1)], ("East", 25)),
        ];

        for TestCase(n, a, b, v, expected) in tests {
            assert_eq!(run(n, a, b, v), expected);
        }
    }
}
