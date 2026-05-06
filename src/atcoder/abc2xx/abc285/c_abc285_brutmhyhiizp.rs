// https://atcoder.jp/contests/abc285/tasks/abc285_c

fn run(s: &str) -> usize {
    s.chars()
        .rev()
        .enumerate()
        .fold(0, |state, (i, c)| {
            state + (c as u8 - 64) as usize * 26_usize.pow(i as u32)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc285_c() {
        let tests = [
            TestCase("AB", 28),
            TestCase("C", 3),
            TestCase("BRUTMHYHIIZP", 10000000000000000),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
