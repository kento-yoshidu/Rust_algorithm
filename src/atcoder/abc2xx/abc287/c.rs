// https://atcoder.jp/contests/abc287/tasks

use std::collections::{HashMap, VecDeque};

fn run(n: usize, m: usize, uv: Option<Vec<(usize, usize)>>) -> &'static str {
    if n - 1 != m {
        return "No";
    }

    let Some(uv) = uv else {
        return "No";
    };

    let mut hash_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for (u, v) in uv {
        hash_map.entry(u).or_default().push(v);
        hash_map.entry(v).or_default().push(u);
    }

    let mut visited = vec![false; n+1];
    visited[1] = true;

    let mut queue = VecDeque::new();
    queue.push_back(1);

    let mut count = 1;

    while let Some(u) = queue.pop_front() {
        for next in hash_map.get(&(u as usize)).unwrap() {
            if visited[*next] {
                continue;
            }

            visited[*next] = true;
            count += 1;
            queue.push_back(*next);
        }
    }

    if count != n {
        return "No";
    }

    let mut deg1 = 0;
    let mut deg2 = 0;

    for i in 1..=n {
        let d = hash_map.get(&i).map_or(0, |v| v.len());
        if d == 1 {
            deg1 += 1;
        } else if d == 2 {
            deg2 += 1;
        } else {
            return "No";
        }
    }

    if deg1 == 2 && deg2 == n - 2 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, usize)>>, &'static str);

    #[test]
    fn abc287_c() {
        let tests = [
            TestCase(4, 3, Some(vec![(1, 3), (4, 2), (3, 2)]), "Yes"),
            TestCase(2, 0, None, "No"),
            TestCase(5, 5, Some(vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 1)]), "No"),
        ];

        for TestCase(n, m, uv, expected) in tests {
            assert_eq!(run(n, m, uv), expected);
        }
    }
}
