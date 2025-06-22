// https://atcoder.jp/contests/abc396/tasks/abc396_b

use std::collections::VecDeque;

fn run(_q: usize, query: Vec<(usize, Option<usize>)>) -> Vec<usize> {
    let mut dequeue = VecDeque::new();
    dequeue.extend(std::iter::repeat(0).take(100));

    let mut ans = Vec::new();

    for (n, x) in query {
        match n {
            1 => {
                dequeue.push_front(x.unwrap());
            },
            2 => {
                ans.push(dequeue.pop_front().unwrap());
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Option<usize>)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![(2, None), (1, Some(4)), (1, Some(3)), (2, None), (2, None), (2, None)], vec![0, 3, 4, 0]),
            TestCase(5, vec![(2, None), (2, None), (2, None), (2, None), (2, None)], vec![0, 0, 0, 0, 0]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
