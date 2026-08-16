// https://atcoder.jp/contests/abc335/tasks/abc335_b

fn run(n: usize) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();

    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k > n {
                    continue;
                }

                ans.push(vec![i, j, k]);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>);

    #[test]
    fn abc335_b() {
        let tests = [
            TestCase(3, vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, 0, 2], vec![0, 0, 3], vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 2], vec![0, 2, 0], vec![0, 2, 1], vec![0, 3, 0],
                        vec![1, 0, 0], vec![1, 0, 1], vec![1, 0, 2], vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 0],
                        vec![2, 0, 0], vec![2, 0, 1], vec![2, 1, 0],
                        vec![3, 0, 0]]),
            TestCase(4, vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, 0, 2], vec![0, 0, 3], vec![0, 0, 4], vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 0], vec![0, 2, 1], vec![0, 2, 2], vec![0, 3, 0], vec![0, 3, 1], vec![0, 4, 0],
                        vec![1, 0, 0], vec![1, 0, 1], vec![1, 0, 2], vec![1, 0, 3], vec![1, 1, 0], vec![1, 1, 1], vec![1, 1, 2], vec![1, 2, 0], vec![1, 2, 1], vec![1, 3, 0],
                        vec![2, 0, 0], vec![2, 0, 1], vec![2, 0, 2], vec![2, 1, 0], vec![2, 1, 1], vec![2, 2, 0],
                        vec![3, 0, 0], vec![3, 0, 1], vec![3, 1, 0],
                        vec![4, 0, 0]]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
