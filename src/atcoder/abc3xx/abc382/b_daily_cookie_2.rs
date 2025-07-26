// https://atcoder.jp/contests/abc382/tasks/abc382_b

fn run(n: usize, d: usize, s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    let mut count = 0;

    for i in (0..n).rev() {
        if chars[i] == '@' && count < d {
            chars[i] = '.';
            count += 1;
        }
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, ".@@.@", ".@..."),
            TestCase(3, 3, "@@@", "..."),
            TestCase(10, 4, "@@@.@@.@@.", "@@@......."),
        ];

        for TestCase(n, d, s, expected) in tests {
            assert_eq!(run(n, d, s), expected);
        }
    }
}
