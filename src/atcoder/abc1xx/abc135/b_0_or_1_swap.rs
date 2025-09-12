// https://atcoder.jp/contests/abc135/tasks/abc135_b

use itertools::Itertools;

fn run(_n: usize, p: Vec<usize>) -> &'static str {
    let vec = p.clone();
    let new_vec: Vec<usize> = p.clone().into_iter().sorted().collect();

    let count = vec.iter()
        .zip(new_vec.iter())
        .filter(|t| {
            t.0 != t.1
        })
        .count();

    if count <= 2 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn abc135_b() {
        let tests = [
            TestCase(5, vec![5, 2, 3, 4, 1], "YES"),
            TestCase(5, vec![2, 4, 3, 5, 1], "NO"),
            TestCase(7, vec![1, 2, 3, 4, 5, 6, 7], "YES"),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
