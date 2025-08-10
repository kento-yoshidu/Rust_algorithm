// https://atcoder.jp/contests/abc126/tasks/abc126_b

fn run(s: usize) -> &'static str {
    let left = s / 100;
    let right = s % 100;

    // 月（01 ~ 12）であるか
    let is_left = 1 <= left && left <= 12;
    let is_right = 1 <= right && right <= 12;

    if is_left && is_right {
        "AMBIGUOUS"
    } else if is_left {
        "MMYY"
    } else if is_right {
        "YYMM"
    } else {
        "NA"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc126_b() {
        let tests = [
            TestCase(1905, "YYMM"),
            TestCase(0519, "MMYY"),
            TestCase(0112, "AMBIGUOUS"),
            TestCase(1700, "NA"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
