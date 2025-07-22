// https://atcoder.jp/contests/abc319/tasks/abc319_a

use std::collections::HashMap;

fn run(s: &str) -> usize {
    let mut map = HashMap::new();

    map.insert("tourist", 3858);
    map.insert("ksun48", 3679);
    map.insert("Benq", 3658);
    map.insert("Um_nik",3648);
    map.insert("apiad", 3638);
    map.insert("Stonefeang", 3630);
    map.insert("ecnerwala", 3613);
    map.insert("mnbvmar", 3555);
    map.insert("newbiedmy", 3516);
    map.insert("semiexp", 3481);

    *map.get(&s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("tourist", 3858),
            TestCase("semiexp", 3481),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
