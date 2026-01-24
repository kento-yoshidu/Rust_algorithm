// https://yukicoder.me/problems/no/24

fn run(_n: usize, a: Vec<([usize; 4], &str)>) -> usize {
    let mut x = [true; 10];

    for (arr, s) in a {
        match s {
            "NO" => {
                for i in arr {
                    x[i] = false;
                }
            },
            "YES" => {
                for i in 0..10 {
                    if !arr.contains(&i) {
                        x[i] = false;
                    }

                }
            },
            _ => unreachable!(),
        }
    }

    x.into_iter()
        .enumerate()
        .find(|(_, b)| *b)
        .map(|(i, _)| i)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<([usize; 4], &'static str)>, usize);

    #[test]
    fn yuki_024() {
        let tests = [
            TestCase(3, vec![([1, 2, 4, 3], "NO"), ([8, 5, 6, 7], "NO"), ([0, 1, 2, 3], "NO")], 9),
            TestCase(2, vec![([1, 2, 3, 4], "YES"), ([4, 5, 6, 7], "YES")], 4),
            TestCase(4, vec![([2, 6, 5, 3], "NO"), ([1, 0, 4, 7], "YES"), ([1, 7, 8, 4], "YES"), ([7, 1, 9, 8], "NO")], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
