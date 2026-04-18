// https://atcoder.jp/contests/abc292/tasks/abc292_b

fn run(n: usize, _q: usize, e: Vec<(usize, usize)>) -> Vec<&'static str> {
    let mut state: Vec<(usize, usize)> = (0..n)
        .map(|i| {
            (i, 0)
        })
        .collect();

    e.into_iter()
        .filter_map(|(t1, t2)| {
            match t1 {
                1 => {
                    state[t2 - 1].1 += 1;
                    None
                },
                2 => {
                    state[t2 - 1].1 += 2;
                    None
                },
                3 => {
                    if state[t2 - 1].1 >= 2 {
                        Some("Yes")
                    } else {
                        Some("No")
                    }
                }
                _ => unreachable!()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<&'static str>);

    #[test]
    fn abc292_b() {
        let tests = [
            TestCase(3, 9, vec![(3, 1), (3, 2), (1, 2), (2, 1), (3, 1), (3, 2), (1, 2), (3, 2), (3, 3)], vec!["No", "No", "Yes", "No", "Yes", "No"]),
            TestCase(5, 11, vec![(3, 1), (1, 2), (3, 2), (1, 3), (1, 3), (3, 3), (1, 4), (2, 4), (3, 4), (1, 5), (3, 5)], vec!["No", "No", "Yes", "Yes", "No"]),
        ];

        for TestCase(n, q, e, expected) in tests {
            assert_eq!(run(n, q, e), expected);
        }
    }
}
