// https://atcoder.jp/contests/abc405/tasks/abc405_c

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;
    let mut sum = 0;

    for n in a {
        ans += n * sum;
        sum += n;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc405_c() {
        let tests = [
            TestCase(3, vec![4, 2, 3], 26),
            TestCase(2, vec![9, 45], 405),
            TestCase(10, vec![7781, 8803, 8630, 9065, 8831, 9182, 8593, 7660, 7548, 8617], 3227530139),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
