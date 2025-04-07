// https://atcoder.jp/contests/abc091/tasks/abc091_b

use std::collections::HashMap;

fn run(_n : i32, s: Vec<&str>, _m: i32, t: Vec<&str>) -> i32 {
    let ans = 0;

    let mut hashmap = HashMap::new();

    for c in s.iter() {
        let count = hashmap.entry(c).or_insert(0);
       *count += 1;
    }

    for c in t.iter() {
        let count = hashmap.entry(c).or_insert(0);

        *count -= 1;
    }

    ans.max(*hashmap.iter()
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, Vec<&'static str>, i32, Vec<&'static str>, i32);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["apple", "orange", "apple"], 1, vec!["grape"], 2),
            TestCase(3, vec!["apple", "orange", "apple"], 5, vec!["apple", "apple", "apple", "apple", "apple"], 1),
            TestCase(1, vec!["voldemort"], 10, vec!["voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort"], 0),
            TestCase(6, vec!["red", "red", "blue", "yellow", "yellow", "red"], 5, vec!["red", "red", "yellow", "green", "blue"], 1),
        ];

        for TestCase(n, s, m, t, expected) in tests {
            assert_eq!(run(n, s, m, t), expected);
        }
    }
}
