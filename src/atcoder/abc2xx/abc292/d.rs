// https://atcoder.jp/contests/abc292/tasks/abc292_d

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, uv: Vec<(usize, usize)>) -> &'static str {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for (u, v) in uv {
        graph.entry(u).or_default().push(v);
        graph.entry(v).or_default().push(u);
    }

    let mut seen = vec![false; n+1];

    for i in 1..=n {
        if seen[i] {
            continue;
        }

        let mut dequeue = VecDeque::new();
        dequeue.push_back(i);
        seen[i] = true;

        let mut v_count = 0;
        let mut e_count = 0;

        while let Some(cur) = dequeue.pop_front() {
            v_count += 1;

            let Some(next) = graph.get(&cur) else {
                continue;
            };

            for next in next {
                e_count += 1;

                if !seen[*next] {
                    seen[*next] = true;
                    dequeue.push_back(*next);

                }
            }
        }

        e_count /= 2;

        if v_count != e_count {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc292_d() {
        let tests = [
            TestCase(3, 3, vec![(2, 3), (1, 1), (2, 3)], "Yes"),
            TestCase(5, 5, vec![(1, 2), (2, 3), (3, 4), (3, 5), (1, 5)], "Yes"),
            TestCase(13, 16, vec![(7, 9), (7, 11), (3, 8), (1, 13), (11, 11), (6, 11), (8, 13), (2, 11), (3, 3), (8, 12), (9, 11), (1, 11), (5, 13), (3, 12), (6, 9), (1, 10)], "No"),
        ];

        for TestCase(n, m, uv, expected) in tests {
            assert_eq!(run(n, m, uv), expected);
        }
    }
}
