// https://atcoder.jp/contests/abc212/tasks/abc212_b

use itertools::Itertools;

pub fn run(x: &str) -> String {
    let vec: Vec<u32> = x.chars().map(|c| c.to_digit(10).unwrap()).collect();

    if vec.iter().all_equal() {
        return String::from("Weak");
    }

    if (0..3).any(|i| {
        vec[i + 1] != (vec[i] + 1) % 10
    }) {
        String::from("Strong")
    } else {
        String::from("Weak")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Weak"), run("7777"));
        assert_eq!(String::from("Strong"), run("0112"));
        assert_eq!(String::from("Weak"), run("9012"));
    }
}
