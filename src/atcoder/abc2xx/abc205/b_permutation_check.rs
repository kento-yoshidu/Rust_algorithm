// https://atcoder.jp/contests/abc205/tasks/abc205_b

use std::collections::HashSet;

pub fn run(n: usize, vec: Vec<usize>) -> String {
    let hash = HashSet::<usize>::from_iter(vec);

    if hash.len() == n {
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
        assert_eq!(String::from("Yes"), run(5, vec![3, 1, 2, 4, 5]));
        assert_eq!(String::from("No"), run(6, vec![3, 1, 4, 1, 5, 2]));
        assert_eq!(String::from("Yes"), run(3, vec![1, 2, 3]));
        assert_eq!(String::from("Yes"), run(1, vec![1]));
    }
}
