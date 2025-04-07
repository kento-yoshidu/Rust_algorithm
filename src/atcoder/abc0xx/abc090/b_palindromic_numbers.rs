// https://atcoder.jp/contests/abc090/tasks/abc090_b

fn run(a: usize, b: usize) -> usize {
    let mut ans = 0;

    for i in a..=b {
        let i1 = i / 10000;
        let i2 = i / 1000 % 10;
        let i3 = i / 10 % 10;
        let i4 = i % 10;

        if i1 == i4 && i2 == i3 {
            ans += 1;
        }
    }

    ans
}

fn calc(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}

fn run2(a: usize, b: usize) -> usize {
    (a..=b)
        .filter(|num| {
            calc(num.to_string())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(11009, 11332, 4),
            TestCase(31415, 92653, 612),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
            assert_eq!(run2(a, b), expected);
        }
    }
}
