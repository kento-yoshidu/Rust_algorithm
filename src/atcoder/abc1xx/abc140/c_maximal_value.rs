// https://atcoder.jp/contests/abc140/tasks/abc140_c

fn run(_n: usize, b: Vec<usize>) -> usize {
    let mut ans = b[0];

    for i in 0..b.len()-1 {
        ans += b[i].min(b[i+1]);
    }

    ans += b.iter().last().unwrap();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc140_c() {
        let tests = [
            TestCase(3, vec![2, 5], 9),
            TestCase(2, vec![3], 6),
            TestCase(6, vec![0, 153, 10, 10, 23], 53),
        ];

        for TestCase(n, b, expected) in tests {
            assert_eq!(run(n, b), expected);
        }
    }
}
