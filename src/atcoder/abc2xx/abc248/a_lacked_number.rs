// https://atcoder.jp/contests/abc248/tasks/abc248_a

fn run(str: &str) -> usize {
    let chars: Vec<char> = str.chars().collect();

    for i in 0..=9 {
        if !chars.contains(&((i as u8 + b'0') as char)) {
            return i as usize
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("012456789", 3),
            TestCase("123456789", 0),
            TestCase("012345678", 9),
            TestCase("459230781", 6),
        ];

        for TestCase(str, expected) in tests {
            assert_eq!(run(str), expected);
        }
    }
}
