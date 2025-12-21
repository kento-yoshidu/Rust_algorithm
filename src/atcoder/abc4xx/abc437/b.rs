// https://atcoder.jp/contests/abc437/tasks/abc437_b

fn run(h: usize, w: usize, n: usize, a: Vec<Vec<usize>>, b: Vec<usize>) -> usize {
    let mut ans = 0;

    for v in a {
        let mut count = 0;

        for num in v {
            if b.contains(&num) {
                count += 1;
            }
        }

        ans = ans.max(count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<Vec<usize>>, Vec<usize>, usize);

    #[test]
    fn abc437_b() {
        let tests = [
            TestCase(3, 4, 5, vec![vec![12, 3, 5, 7], vec![6, 10, 11, 9], vec![1, 2, 4, 8]], vec![2, 4, 9, 6, 11], 3),
            TestCase(3, 5, 2, vec![vec![81, 63, 31, 16, 15], vec![30, 3, 6, 54, 24], vec![26, 41, 48, 64, 66]], vec![44, 79], 0),
            TestCase(3, 5, 12, vec![vec![78, 19, 70, 58, 83], vec![12, 30, 80, 20, 27], vec![48, 71, 8, 43, 82]], vec![82, 30, 43, 8, 80, 70, 20, 78, 12, 71, 19, 48], 5),
        ];

        for TestCase(h, w, n, a, b, expected) in tests {
            assert_eq!(run(h, w, n, a, b), expected);
        }
    }
}