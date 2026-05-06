// https://atcoder.jp/contests/abc306/tasks/abc306_b

fn run(a: Vec<usize>) -> usize {
    let mut ans = 0;

    for (i, num) in a.iter().enumerate() {
        ans += num << i;
    }

    ans
}

fn run2(a: Vec<usize>) -> usize {
    a.into_iter()
        .enumerate()
        .fold(0, |sum, (i, v)| { sum + (v << i) })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, usize);

    #[test]
    fn abc306_b() {
        let tests = [
            TestCase(vec![1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 13),
            TestCase(vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0], 766067858140017173),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
