// https://atcoder.jp/contests/arc032/tasks/arc032_1

fn run(n: usize) -> &'static str {
    if n == 1 {
        return "BOWWOW";
    }

    let sum = n*(n+1) / 2;

    for i in 2..=sum-1 {
        if sum % i == 0 {
            return "BOWWOW";
        }
    }

    "WANWAN"
}

fn run2(n: usize) -> &'static str {
    if n == 2 {
        "WANWAN"
    } else {
        "BOWWOW"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, "WANWAN"),
            TestCase(5, "BOWWOW"),
            TestCase(1, "BOWWOW"),
            TestCase(999, "BOWWOW"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
