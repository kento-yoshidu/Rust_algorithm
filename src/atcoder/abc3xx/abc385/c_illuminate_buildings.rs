// https://atcoder.jp/contests/abc385/tasks/abc385_c

fn run(n: usize, h: Vec<usize>) -> usize {
    let mut ans = 0;

    for start in 0..n {
        for j in 1..n {
            let mut interval = j;

            let mut count = 1;

            while start + interval < n {
                if h[start] == h[start+interval] {
                    count += 1;
                } else {
                    break;
                }

                interval += j;
            }

            ans = ans.max(count);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, vec![5, 7, 5, 7, 7, 5, 7, 7], 3),
            TestCase(10, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 1),
            TestCase(32, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5], 3),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
