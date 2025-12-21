//https://atcoder.jp/contests/abc211/tasks/abc211_b

use itertools::Itertools;

fn run(a: &Vec<&str>) -> &'static str {
    use std::collections::HashSet;

    let set: HashSet<&&str> = a.iter().collect();

    if set.len() == 4 {
        "Yes"
    } else {
        "No"
    }
}

fn run2(a: &Vec<&str>) -> &'static str {
    let a: Vec<&str> = a.iter().cloned().sorted().dedup().collect();

    if a.len() == 4 {
        "Yes"
    } else {
        "No"
    }
}

fn run3(a: &Vec<&str>) -> &'static str {
    if a.into_iter().all_unique() {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, &'static str);

    #[test]
    fn abc211_b() {
        let tests = [
            TestCase(vec!["3B", "HR", "2B", "H"], "Yes"),
            TestCase(vec!["2B", "3B", "HR", "3B"], "No"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(&a), expected);
            assert_eq!(run2(&a), expected);
            assert_eq!(run3(&a), expected);
        }
    }
}
