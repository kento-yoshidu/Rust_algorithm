// https://atcoder.jp/contests/abc187/tasks/abc187_c

use std::collections::HashSet;

fn run(_n: usize, a: Vec<&str>) -> String {
    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();

    for s in a {
        if s.starts_with('!') {
            set_b.insert(s.to_string());
        } else {
            set_a.insert(s.to_string());
        }
    }

    set_a.iter()
        .find(|str| set_b.contains(&format!("!{}", str)))
        .map(|s| s.to_string())
        .unwrap_or("satisfiable".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            // Todo?
            // TestCase(6, vec!["a", "!a", "b", "!c", "d", "!d"], "a"),
            TestCase(10, vec!["red", "red", "red", "!orange", "yellow", "!blue", "cyan", "!green", "brown", "!gray"], "satisfiable"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
