// https://atcoder.jp/contests/abc371/tasks/abc371_b

fn run(n: usize, _m: usize, ab: Vec<(usize, char)>) -> Vec<&'static str> {
    let mut arr = vec![false; n];

    ab.into_iter()
        .map(|(a, b)| {
            match b {
                'F' => {
                    "No"
                },
                'M' => {
                    if arr[a-1] == false {
                        arr[a-1] = true;
                        "Yes"
                    } else {
                        "No"
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

    struct TestCase(usize, usize, Vec<(usize, char)>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 4, vec![(1, 'M'), (1, 'M'), (2, 'F'), (2, 'M')], vec!["Yes", "No", "No", "Yes"]),
            TestCase(4, 7, vec![(2, 'M'), (3, 'M'), (1, 'F'), (4, 'F'), (4, 'F'), (1, 'F'), (2, 'M')], vec!["Yes", "Yes", "No", "No", "No", "No", "No"]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
