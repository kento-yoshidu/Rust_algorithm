// https://atcoder.jp/contests/abc170/tasks/abc170_b

fn run(x: i32, y: i32) -> &'static str {
    for i in 0..=x {
        let kame = i;
        let tsuru = x - i;

        if kame*4 + tsuru*2 == y {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, &'static str);

    #[test]
    fn abc170_b() {
        let tests = [
            TestCase(3, 8, "Yes"),
            TestCase(2, 100, "No"),
            TestCase(25, 100, "Yes"),
            TestCase(1, 2, "Yes"),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
