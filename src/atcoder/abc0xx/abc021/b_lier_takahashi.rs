// https://atcoder.jp/contests/abc021/tasks/abc021_b

fn run(_n: usize, a: usize, b: usize, _k: usize, p: Vec<usize>) -> &'static str {
    let mut vec = vec![a, b];

    vec.append(&mut p.clone());
    vec.sort();
    vec.dedup();

    if vec.len() == p.len() + 2 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 1, 5, 3, vec![3, 4, 2], "Yes"),
            TestCase(7, 1, 3, 4, vec![2, 4, 2, 7], "No"),
            TestCase(4, 1, 4, 3, vec![2, 1, 3], "No"),
            TestCase(4, 1, 4, 3, vec![2, 4, 3], "No"),
            TestCase(20, 1, 4, 12, vec![2, 3, 5, 7, 8, 9, 10, 11, 12, 15, 13, 14], "Yes"),
        ];

        for TestCase(n, a, b, k, p, expected) in tests {
            assert_eq!(run(n, a, b, k, p), expected);
        }
    }
}
