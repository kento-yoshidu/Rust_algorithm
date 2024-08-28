// https://atcoder.jp/contests/abc361/tasks/abc361_b

fn run(a: isize, b: isize, c: isize, d: isize, e: isize, f: isize, g: isize, h: isize, i: isize, j: isize, k: isize, l: isize) -> &'static str {
    let x = (a.max(g), b.max(h), c.max(i));
    let y = (d.min(j), e.min(k), f.min(l));

    if (y.0 - x.0) * (y.1 - x.1) * (y.2 - x.2) > 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 0, 0, 4, 5, 6, 2, 3, 4, 5, 6, 7, "Yes"),
            TestCase(0, 0, 0, 2, 2, 2, 0, 0, 2, 2, 2, 4, "No"),
            TestCase(0, 0, 0, 1000, 1000, 1000, 10, 10, 10, 100, 100, 100, "Yes"),
        ];

        for TestCase(a, b, c, d, e, f, g, h, i, j, k, l, expected) in tests {
            assert_eq!(run(a, b, c, d, e, f, g, h, i, j, k, l), expected);
        }
    }
}
