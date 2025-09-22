// https://yukicoder.me/problems/no/5

use itertools::Itertools;

fn rec(i: usize, rest: usize, w: Vec<usize>) -> usize {
    if i >= w.len() || rest < w[i] {
        i
    } else {
        rec(i+1, rest - w[i], w)
    }
}

fn run(l: usize, _n: usize, w: Vec<usize>) -> usize {
    let sorted: Vec<usize> = w.into_iter().sorted().collect();

    rec(0, l, sorted)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn yuki_005() {
        let tests = [
            TestCase(16, 3, vec![10, 5, 7], 2),
            TestCase(100, 10, vec![14, 85, 77, 26, 50, 45, 66, 79, 10, 3], 5),
            TestCase(1, 1, vec![1], 1),
        ];

        for TestCase(l, n, w, expected) in tests {
            assert_eq!(run(l, n, w), expected);
        }
    }
}
