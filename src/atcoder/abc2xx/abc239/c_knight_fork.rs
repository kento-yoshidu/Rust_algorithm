// https://atcoder.jp/contests/abc239/tasks/abc239_c

fn calc(a: isize, b: isize, c: isize, d: isize) -> isize {
    (a - c).pow(2) + (b - d).pow(2)
}

fn run(x1: isize, y1: isize, x2: isize, y2: isize) -> &'static str {
    for i in x1-2..x1+3 {
        for j in y1-2..y1+3 {
            if calc(i, j, x1, y1) == 5 && calc(i, j, x2, y2) == 5 {
                return "Yes"
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, &'static str);

    #[test]
    fn abc239_c() {
        let tests = [
            TestCase(0, 0, 3, 3, "Yes"),
            TestCase(0, 1, 2, 3, "No"),
            TestCase(1000000000, 1000000000, 999999999, 999999999, "Yes"),
        ];

        for TestCase(x1, y1, x2, y2, expected) in tests {
            assert_eq!(run(x1, y1, x2, y2), expected);
        }
    }
}
