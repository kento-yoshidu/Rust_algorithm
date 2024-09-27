// https://atcoder.jp/contests/abc123/tasks/abc123_a

fn run(a: usize, b: usize, c: usize, d: usize, e: usize, k: usize) -> &'static str {
    let vec = vec![a, b, c, d, e];

    let max = vec.iter().max().unwrap();
    let min = vec.iter().min().unwrap();

    if (max - min) <= k {
        "Yay!"
    } else {
        ":("
    }
}

fn run2(a: usize, _b: usize, _c: usize, _d: usize, e: usize, k: usize) -> &'static str {
    if e - a <= k {
        "Yay!"
    } else {
        ":("
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 4, 8, 9, 15, "Yay!"),
            TestCase(15, 18, 26, 35, 36, 18, ":("),
        ];

        for TestCase(a, b, c ,d, e, k, expected) in tests {
            assert_eq!(run(a, b, c, d, e, k), expected);
            assert_eq!(run2(a, b, c, d, e, k), expected);
        }
    }
}
