// https://atcoder.jp/contests/abc456/tasks/abc456_c

fn run(s: &str) -> usize {
    let s: Vec<char> = s.chars().collect();
    let len = s.len();

    let mut ans = 0;
    let mut r = 0;

    for l in 0..len {
        while r < len && (r <= l || s[r] != s[r-1]) {
            r += 1;
        }

        ans = (ans + r - l) % 998244353;

        if l == r {
            r += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc456_c() {
        let tests = [
            TestCase("abbc", 6),
            TestCase("cabcabcbcaccacbcbcaabacbacaabccacbccbcacbacbacabcacabcaccaaaaabababcbabacaccabbcacbcbcbcababcbcbabca", 760),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
