// https://atcoder.jp/contests/abc424/tasks/abc424_c

use std::collections::{HashMap, VecDeque};

fn run(n: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut map = HashMap::new();
    let mut seen = vec![false; n];

    let mut queue = VecDeque::new();

    for (i, (a, b)) in ab.into_iter().enumerate() {
        if a == 0 && b == 0 {
            queue.push_back(i);
            seen[i] = true;
        } else {
            map.entry(a-1).or_insert_with(|| Vec::new()).push(i);
            map.entry(b-1).or_insert_with(|| Vec::new()).push(i);
        }
    }

    let mut ans = 0;

    while let Some(cur) = queue.pop_front() {
        ans += 1;

        let Some(next) = map.get(&cur) else {
            continue;
        };

        for n in next {
            if !seen[*n] {
                seen[*n] = true;
                queue.push_back(*n);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc424_c() {
        let tests = [
            TestCase(6,vec![(0, 0), (1, 3), (3, 2), (5, 5), (4, 6), (6, 4)], 3),
            TestCase(4, vec![(0, 0), (0, 0), (0, 0), (0, 0)], 4),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
