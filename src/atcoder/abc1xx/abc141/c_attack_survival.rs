// https://atcoder.jp/contests/abc141/tasks/abc141_c

fn run(n: usize, k: isize, q: isize, a: Vec<usize>) -> Vec<&'static str> {
    let mut vec = vec![0; n];

    for num in a {
        vec[num-1] += 1;
    }

    vec.into_iter()
        .map(|num| {
            if k - (q - num) <= 0 {
                "No"
            } else {
                "Yes"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, isize, Vec<usize>, Vec<&'static str>);

    #[test]
    fn abc141_c() {
        let tests = [
            TestCase(6, 3, 4, vec![3, 1, 3, 2], vec!["No", "No", "Yes", "No", "No", "No"]),
            TestCase(6, 5, 4, vec![3, 1, 3, 2], vec!["Yes", "Yes", "Yes", "Yes", "Yes", "Yes"]),
            TestCase(10, 13, 15, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9], vec!["No", "No", "No", "No", "Yes", "No", "No", "No", "Yes", "No"]),
        ];

        for TestCase(n, k, q, a, expected) in tests {
            assert_eq!(run(n, k, q, a), expected);
        }
    }
}
