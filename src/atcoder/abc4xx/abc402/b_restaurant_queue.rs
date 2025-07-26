// https://atcoder.jp/contests/abc402/tasks/abc402_b

use std::collections::VecDeque;

fn run(_q: usize, query: Vec<(usize, Option<usize>)>) -> Vec<usize> {
    let mut ans = Vec::new();
    let mut queue = VecDeque::new();

    for (n, q) in query {
        match n {
            1 => {
                queue.push_back(q.unwrap());
            },
            2 => {
                ans.push(queue.pop_front().unwrap());
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
            TestCase(6, vec![(1, Some(3)), (1, Some(1)), (1, Some(15)), (2, None), (1, Some(3)), (2, None)], vec![3, 1]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
