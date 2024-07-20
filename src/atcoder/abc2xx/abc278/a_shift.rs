// https://atcoder.jp/contests/abc278/tasks/abc278_a

use std::collections::VecDeque;

fn run(_n: usize, k: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec_deque = VecDeque::from(a);

    for _i in 0..k {
        vec_deque.pop_front();
        vec_deque.push_back(0);
    }

    vec_deque.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec![2, 7, 8], vec![8, 0, 0]),
            TestCase(3, 4, vec![9, 9, 9], vec![0, 0, 0]),
            TestCase(9, 5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![6, 7, 8, 9, 0, 0, 0, 0, 0]),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
