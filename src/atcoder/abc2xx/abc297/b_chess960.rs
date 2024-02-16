// https://atcoder.jp/contests/abc297/tasks/abc297_b

use itertools::Itertools;

fn run(s: &str) -> &'static str {
    let b_pos: Vec<usize> = s.chars().positions(|c| c == 'B').collect();

    if b_pos[0] % 2 == b_pos[1] % 2 {
        return "No";
    }

    let k_pos = s.chars().position(|c| c == 'K').unwrap();

    if b_pos[0] < k_pos && k_pos < b_pos[1] {
        "Yes"
    } else {
        "No"
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("RNBQKBNR", "Yes"),
            TestCase("KRRBBNNQ", "No"),
            TestCase("BRKRBQNN", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
