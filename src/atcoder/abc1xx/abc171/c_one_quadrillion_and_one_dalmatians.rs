// https://atcoder.jp/contests/abc171/tasks/abc171_c

fn run(n: usize) -> String {
    let mut n = n;

    let mut vec = Vec::new();

    loop {
        if n == 0 {
            break;
        }

        n -= 1;
        vec.push(('a' as u8 + (n % 26) as u8) as char);
        n /= 26;
    }

    vec.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, "b"),
            TestCase(27, "aa"),
            TestCase(123456789, "jjddja")
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
