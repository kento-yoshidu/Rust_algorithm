// https://atcoder.jp/contests/abc001/tasks/abc001_2

fn run(m: usize) -> String {
    if m < 100 {
        String::from("00")
    } else if 100 <= m && m <= 5000 {
        format!("{:02}", m / 100)
    } else if 6000 <= m && m <= 30000 {
        format!("{}", m / 1000 + 50)
    } else if 35000 <= m && m <= 70000 {
        format!("{}", ((m / 1000) - 30) / 5 + 80)
    } else {
        String::from("89")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(15000, "65"),
            TestCase(75000, "89"),
            TestCase(200, "02"),
            TestCase(2000, "20"),
            TestCase(40000, "82"),
        ];

        for TestCase(m, expected) in tests {
            assert_eq!(run(m), expected);
        }
    }
}
