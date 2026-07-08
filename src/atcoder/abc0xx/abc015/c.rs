// https://atcoder.jp/contests/abc015/tasks/abc015_3

use std::collections::VecDeque;

fn run(n: usize, _k: usize, t: Vec<Vec<usize>>) -> &'static str {
    let mut queue = VecDeque::new();

    for &n in &t[0] {
        queue.push_back((0, vec![n]));
    }

    while let Some((i,  vec)) = queue.pop_front() {
        if i + 1 == n {
            if vec.iter().fold(0, |acc, &x| acc ^ x) == 0 {
                return "Found";
            }
            continue;
        }

        let next_values = t[i + 1].clone();

        for next in next_values {
            let mut new_vec = vec.clone();
            new_vec.push(next);
            queue.push_back((i + 1, new_vec));
        }
    }

    "Nothing"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, &'static str);

    #[test]
    fn abc015_c() {
        let tests = [
            TestCase(3, 4, vec![vec![1, 3, 5, 17], vec![2, 4, 2, 3], vec![1, 3, 2, 9]], "Found"),
            TestCase(5, 3, vec![vec![89, 62, 15], vec![44, 36, 17], vec![4, 24, 24], vec![25, 98, 99], vec![66, 33, 57]], "Nothing"),
        ];

        for TestCase(n, k, t, expected) in tests {
            assert_eq!(run(n, k, t), expected);
        }
    }
}
