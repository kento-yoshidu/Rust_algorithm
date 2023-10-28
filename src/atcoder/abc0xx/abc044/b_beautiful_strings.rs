// https://atcoder.jp/contests/abc044/tasks/abc044_b

use itertools::Itertools;

pub fn run(w: String) -> String {
    let hashmap = w.chars().counts();

    if hashmap.iter().all(|(_, value)| {
        value % 2 == 0
    }) {
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
        assert_eq!(String::from("Yes"), run(String::from("abaccaba")));
        assert_eq!(String::from("No"), run(String::from("hthth")));
    }
}
