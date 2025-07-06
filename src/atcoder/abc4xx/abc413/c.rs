// https://atcoder.jp/contests/abc413/tasks/abc413_c

use std::collections::VecDeque;

fn run(_q: usize, query: Vec<(usize, usize, Option<usize>)>) -> Vec<usize> {
    let mut deque = VecDeque::new();

    let mut ans = Vec::new();

    for (n, a, b) in query {
        match n {
            1 => {
                deque.push_back((a, b.unwrap()));
            },
            2 => {
                if deque[0].0 == a {
                    ans.push(deque[0].0 * deque[0].1);
                    deque.pop_front();
                } else if deque[0].0 > a {
                    ans.push(deque[0].1 * a);
                    deque[0].0 -= a;
                } else {
                    let mut total = 0;
                    let mut rest = a;

                    loop {
                        if deque[0].0 < rest {
                            total += deque[0].0 * deque[0].1;
                            rest -= deque[0].0;
                            deque.pop_front();
                        } else {
                            total += deque[0].1 * rest;
                            deque[0].0 -= rest;
                            break;
                        }
                    }

                    ans.push(total);
                }
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize, Option<usize>)>, Vec<usize>);

    #[test]
    fn abc413_c() {
        let tests = [
            TestCase(5, vec![(1, 2, Some(3)), (1, 4, Some(5)), (2, 3, None), (1, 6, Some(2)), (2, 5, None)], vec![11, 19]),
            TestCase(10, vec![(1, 75, Some(22)), (1, 81, Some(72)), (1, 2, Some(97)), (1, 84, Some(82)), (1, 2, Some(32)), (1, 39, Some(57)), (2, 45, None), (1, 40, Some(16)), (2, 32, None), (2, 42, None)], vec![990, 804, 3024]),
            TestCase(10, vec![(1, 160449218, Some(954291757)), (2, 17217760, None), (1, 353195922, Some(501899080)), (1, 350034067, Some(910748511)), (1, 824284691, Some(470338674)), (2, 180999835, None), (1, 131381221, Some(677959980)), (1, 346948152, Some(208032501)), (1, 893229302, Some(506147731)), (2, 298309896, None)], vec![16430766442004320, 155640513381884866, 149721462357295680]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
