// https://atcoder.jp/contests/abc066/tasks/arc077_a

use std::collections::VecDeque;

fn run(n: usize, a: Vec<usize>) -> VecDeque<usize> {
    let mut deque = VecDeque::new();

    for (i, num) in a.into_iter().enumerate() {
        if i % 2 == 0 {
            deque.push_back(num);
        } else {
            deque.push_front(num);
        }
    }

    if n % 2 == 0 {
        deque
    } else {
        deque.make_contiguous().reverse();
        deque
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, VecDeque<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![1, 2, 3, 4], VecDeque::from([4, 2, 1, 3])),
            TestCase(3, vec![1, 2, 3], VecDeque::from([3, 1, 2])),
            TestCase(1, vec![1000000000], VecDeque::from([1000000000])),
            TestCase(6, vec![0, 6, 7, 6, 7, 0], VecDeque::from([0, 6, 6, 0, 7, 7])),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
