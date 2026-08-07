// https://atcoder.jp/contests/abc023/tasks/abc023_b

fn run(n: usize, s: &str) -> isize {
    let n = n / 2;

    let mut str = String::from("b");

    for i in 1..=n {
        match i % 3 {
            0 => str = format!("b{}b", str),
            1 => str = format!("a{}c", str),
            2 => str = format!("c{}a", str),
            _ => unreachable!(),
        }
    }

    if s == str {
        n as isize
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, "abc", 1),
            TestCase(6, "abcabc", -1),
            TestCase(7, "atcoder", -1),
            TestCase(19, "bcabcabcabcabcabcab", 9),
            TestCase(1, "b", 0),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(expected, run(n, s));
        }
    }
}
