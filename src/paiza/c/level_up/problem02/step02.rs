// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_string_step2

use itertools::Itertools;

fn run(a: char, s: &str) -> &'static str {
    if s.chars().contains(&a) {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, &'static str, &'static str);

    #[test]
    fn paiza_c_level_up_02_step02() {
        let tests = [
            TestCase('Z', "Kirishima", "NO"),
            TestCase('a', "paiza", "YES"),
        ];

        for TestCase(a, s, expected) in tests {
            assert_eq!(run(a, s), expected);
        }
    }
}
