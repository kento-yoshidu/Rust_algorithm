// https://atcoder.jp/contests/agc027/tasks/agc027_a

use itertools::Itertools;

fn rec(i: usize, rest: usize, a: &Vec<usize>) -> usize {
    if i+1 == a.len() {
        if rest == a[i] {
            i + 1
        } else {
            i
        }
    } else if rest < a[i] {
        i
    } else {
        rec(i+1, rest - a[i], &a)
    }
}

fn run(_n: usize, x: usize, a: Vec<usize>) -> usize {
    let a = a.into_iter().sorted().collect();

    rec(0, x, &a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 70, vec![20, 30, 10], 2),
            TestCase(3, 10, vec![20, 30, 10], 1),
            TestCase(4, 1111, vec![1, 10, 100, 1000], 4),
            TestCase(2, 10, vec![20, 20], 0),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
