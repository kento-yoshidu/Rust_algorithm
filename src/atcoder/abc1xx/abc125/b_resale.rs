// https://atcoder.jp/contests/abc125/tasks/abc125_b

fn run(n: usize, v: Vec<isize>, c: Vec<isize>) -> isize {
    let mut ans = 0;

    for bit in 1..(1 << n) {
        let mut total = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                total += v[i] - c[i]
            }
        }

        ans = ans.max(total)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>, isize);

    #[test]
    fn abc125_b() {
        let tests = [
            TestCase(3, vec![10, 2, 5], vec![6, 3, 4], 5),
            TestCase(4, vec![13, 21, 6, 19], vec![11, 30, 6, 15], 6),
            TestCase(1, vec![1], vec![50], 0),
        ];

        for TestCase(n, v, c, expected) in tests {
            assert_eq!(run(n, v, c), expected);
        }
    }
}
