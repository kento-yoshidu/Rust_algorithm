// https://atcoder.jp/contests/abc399/tasks/abc399_c

use std::collections::{HashMap, VecDeque};

fn run(n: usize, m: usize, vu: Option<Vec<(usize, usize)>>) -> usize {
    if vu.is_none() {
        return 0;
    }

    let mut map = HashMap::new();

    for (v, u) in vu.unwrap() {
        map.entry(v).or_insert_with(|| Vec::new()).push(u);
        map.entry(u).or_insert_with(|| Vec::new()).push(v);
    }

    let mut visited = vec![false; n+1];
    let mut count = 0;

    for i in 1..=n {
        if visited[i] {
            continue;
        }

        count += 1;

        let mut queue = VecDeque::new();
        visited[i] = true;
        queue.push_back(i);

        while let Some(v) = queue.pop_front() {
            let Some(next) = map.get(&v) else {
                continue;
            };

            for n in next {
                if visited[*n] {
                    continue;
                }

                visited[*n] = true;
                queue.push_back(*n);
            }
        }
    }

    m - (n - count)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, usize)>>, usize);

    #[test]
    fn abc399_c() {
        let tests = [
            TestCase(4, 4, Some(vec![(1, 2), (1, 3), (2, 4), (3, 4)]), 1),
            TestCase(5, 0, None, 0),
            TestCase(10, 10, Some(vec![ (7, 9), (4, 6), (6, 10), (2, 5), (5, 6), (5, 9), (6, 8), (4, 8), (1, 5), (1, 4)]), 2),
        ];

        for TestCase(n, m, vu, expected) in tests {
            assert_eq!(run(n, m, vu), expected);
        }
    }
}
