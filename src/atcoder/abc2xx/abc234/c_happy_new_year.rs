// https://atcoder.jp/contests/abc234/tasks/abc234_c

fn run(k: usize) -> String {
    let s = format!("{:b}", k);

    s.chars()
        .map(|c| {
            if c == '1' {
                '2'
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc234_c() {
        let tests = [
            TestCase(3, "22"),
            TestCase(11, "2022"),
            TestCase(923423423420220108, "220022020000202020002022022000002020002222002200002022002200"),
        ];

        for TestCase(k, expected) in tests {
            assert_eq!(run(k), expected);
        }
    }
}
