// https://yukicoder.me/problems/no/998

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    if [a, b, c, d]
        .into_iter()
        .sorted()
        .tuple_windows()
        .all(|(x, y)| y - x == 1) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn yuki_998() {
        let tests = [
            TestCase(3, 1, 2, 4, "Yes"),
            TestCase(0, 2, 2, 4, "No"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
