//https://atcoder.jp/contests/abc211/tasks/abc211_b

pub fn run(vec: Vec<&str>) -> String {
    use std::collections::HashSet;

    let set: HashSet<&&str> = vec.iter().collect();

    if set.len() == 4 {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

pub fn run2(vec: &mut Vec<&str>) -> String {
    vec.sort();
    vec.dedup();

    if vec.len() == 4 {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

pub fn run3(a: Vec<&str>) -> String {
    use itertools::Itertools;

    if a.iter().all_unique() {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(vec!["3B", "HR", "2B", "H"]));
        assert_eq!(String::from("No"), run(vec!["2B", "3B", "HR", "3B"]));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(&mut vec!["3B", "HR", "2B", "H"]));
        assert_eq!(String::from("No"), run2(&mut vec!["2B", "3B", "HR", "3B"]));
    }

    #[test]
    fn test3() {
        assert_eq!(String::from("Yes"), run3(vec!["3B", "HR", "2B", "H"]));
        assert_eq!(String::from("No"), run3(vec!["2B", "3B", "HR", "3B"]));
    }
}
