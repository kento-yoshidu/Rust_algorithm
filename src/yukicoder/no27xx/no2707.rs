// https://yukicoder.me/problems/no/2707

fn run(_n: usize, s: &str) -> String {
    let mut ans = [0; 26];

    for c in s.chars() {
        ans[(c as u8 - 65) as usize] += 1;
    }

    ans.into_iter().map(|a| a.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn yuki_2707() {
        let tests = [
            TestCase(5, "ABCDE", "11111000000000000000000000"),
            TestCase(12, "AAAAAAAAAABB", "102000000000000000000000000"),
            TestCase(1, "Z", "00000000000000000000000001"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
