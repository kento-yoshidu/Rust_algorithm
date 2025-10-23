// https://atcoder.jp/contests/joi2026yo1a/tasks/joi2026_yo1a_d

use itertools::Itertools;

fn run(n: usize, s: Vec<Vec<char>>) -> &'static str {
    for a in s.iter() {
        if a.iter().all_equal() {
            return "Yes";
        }
    }

    for i in 0..n {
        let mut arr = Vec::new();

        for j in 0..n {
            arr.push(s[j][i]);
        }

        if arr.into_iter().all_equal() {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<char>>, &'static str);

    #[test]
    fn joi2026yo1a_d() {
        let tests = [
            TestCase(3, vec![vec!['o', 'x', 'x'], vec!['o', 'o', 'x'], vec!['o', 'x', 'o']], "Yes"),
            TestCase(4, vec![vec!['o', 'x', 'x', 'x'], vec!['x', 'o', 'x', 'x'], vec!['x', 'x', 'o', 'x'], vec!['x', 'x', 'x', 'o']], "No"),
            TestCase(6, vec![vec!['x', 'x', 'o', 'o', 'x', 'x'], vec!['x', 'o', 'x', 'x', 'o', 'x'], vec!['o', 'x', 'x', 'x', 'x', 'o'], vec!['o', 'x', 'x', 'x', 'x', 'o'], vec!['x', 'o', 'x', 'x', 'o', 'x'], vec!['x', 'x', 'o', 'o', 'x', 'x']], "No"),
            TestCase(10, vec![vec!['o', 'x', 'o', 'o', 'o', 'x', 'o', 'x', 'o', 'x'], vec!['o', 'o', 'x', 'o', 'o', 'x', 'o', 'o', 'o', 'x'], vec!['x', 'o', 'o', 'x', 'o', 'o', 'o', 'x', 'x', 'x'], vec!['o', 'o', 'o', 'x', 'o', 'o', 'x', 'o', 'x', 'x'], vec!['o', 'o', 'o', 'o', 'o', 'o', 'o', 'x', 'o', 'x'], vec!['x', 'x', 'x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'], vec!['o', 'o', 'o', 'x', 'o', 'o', 'o', 'o', 'o', 'x'], vec!['o', 'o', 'o', 'x', 'o', 'o', 'o', 'o', 'o', 'x'], vec!['o', 'o', 'o', 'x', 'o', 'o', 'x', 'x', 'o', 'x'], vec!['o', 'x', 'o', 'o', 'o', 'o', 'x', 'o', 'o', 'x']], "Yes")
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
