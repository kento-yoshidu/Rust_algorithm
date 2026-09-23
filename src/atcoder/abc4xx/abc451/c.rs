// https://atcoder.jp/contests/abc451/tasks/abc451_c

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn run(q: usize, query: Vec<(usize, usize)>) -> Vec<usize> {
    let mut queue = BinaryHeap::new();

    query.into_iter()
        .map(|(x, h)| {
            match x {
                1 => {
                    queue.push(Reverse(h));
                    queue.len()

                },
                2 => {
                    while let Some(&Reverse(i)) = queue.peek() {
                        if i <= h {
                            queue.pop();
                        } else {
                            break;
                        }
                    }

                    queue.len()
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc451_c() {
        let tests = [
            TestCase(5, vec![(1, 5), (1, 7), (1, 8), (2, 7), (1, 3)], vec![1, 2, 3, 1, 2]),
            TestCase(12, vec![(2, 256601193), (1, 85138616), (1, 202564041), (2, 276477192), (1, 55551662), (1, 170271057), (2, 754166580), (1, 854388209), (1, 772036624), (2, 651124113), (1, 301137866), (2, 290875185)], vec![ 0, 1, 2, 0, 1, 2, 0, 1, 2, 2, 3, 3]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
     }
}
