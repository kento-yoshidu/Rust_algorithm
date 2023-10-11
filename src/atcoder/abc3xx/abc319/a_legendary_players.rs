// https://atcoder.jp/contests/abc319/tasks/abc319_a

use std::collections::HashMap;

pub fn run(s: String) -> usize {
    let mut map = HashMap::new();

    map.insert(String::from("tourist"), 3858);
    map.insert(String::from("ksun48"), 3679);
    map.insert(String::from("Benq"), 3658);
    map.insert(String::from("Um_nik"),3648);
    map.insert(String::from("apiad"), 3638);
    map.insert(String::from("Stonefeang"), 3630);
    map.insert(String::from("ecnerwala"), 3613);
    map.insert(String::from("mnbvmar"), 3555);
    map.insert(String::from("newbiedmy"), 3516);
    map.insert(String::from("semiexp"), 3481);

    *map.get(&s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3858, run(String::from("tourist")));
        assert_eq!(3481, run(String::from("semiexp")));
    }
}
