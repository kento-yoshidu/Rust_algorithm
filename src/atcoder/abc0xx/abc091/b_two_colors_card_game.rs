// https://atcoder.jp/contests/abc091/tasks/abc091_b

use std::collections::HashMap;

pub fn run(_n : i32, s: Vec<&str>, _m: i32, t: Vec<&str>) -> i32 {
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

    #[test]
    fn test() {
        assert_eq!(2, run(3, vec!["apple", "orange", "apple"], 1, vec!["grape"]));
        assert_eq!(1, run(3, vec!["apple", "orange", "apple"], 5, vec!["apple", "apple", "apple", "apple", "apple"]));
        assert_eq!(0, run(1, vec!["voldemort"], 10, vec!["voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort", "voldemort"]));
        assert_eq!(1, run(6, vec!["red", "red", "blue", "yellow", "yellow", "red"], 5, vec!["red", "red", "yellow", "green", "blue"]));
    }
}
