// https://atcoder.jp/contests/abc324/tasks/abc324_a

use itertools::Itertools;

pub fn run(_n: usize, a: Vec<usize>) -> String {
    if a.iter().all_equal() {
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
        assert_eq!(String::from("No"), run(3, vec![3, 2, 4]));
        assert_eq!(String::from("Yes"), run(4, vec![3, 3, 3, 3]));
        assert_eq!(String::from("No"), run(4, vec![73, 8, 55, 26, 97, 48, 37, 47, 35, 55]));
    }
}
