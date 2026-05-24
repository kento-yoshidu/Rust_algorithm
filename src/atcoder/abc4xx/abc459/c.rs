// https://atcoder.jp/contests/abc459/tasks/abc459_c

fn run(n: usize, q: usize, query: Vec<(usize, usize)>) -> Vec<usize> {
    // 各マスの高さ
    let mut height = vec![0; n];

    // 「高さvになった回数」
    let mut count = vec![0; q + 1];

    // 消した回数
    let mut deleted = 0;

    query.into_iter()
        .filter_map(|(q, x)| {
            match q {
                1 => {
                    let i = x - 1;

                    height[i] += 1;
                    count[height[i]] += 1;

                    if count[height[i]] == n {
                        deleted = height[i];
                    }

                    None
                },
                2 => {
                    let k = deleted + x;

                    if k > q {
                        Some(0)
                    } else {
                        Some(count[k])
                    }
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc459_c() {
        let tests = [
            TestCase(3, 7, vec![(1, 1), (1, 3), (1, 3), (2, 1), (2, 2), (1, 2), (2, 1)], vec![2, 1, 1]),
        ];

        for TestCase(n, q, query, expected) in tests {
            assert_eq!(run(n, q, query), expected);
        }
    }
}
