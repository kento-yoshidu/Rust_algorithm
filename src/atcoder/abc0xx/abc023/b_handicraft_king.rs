// https://atcoder.jp/contests/abc023/tasks/abc023_b

pub fn run(n: usize, s: String) -> isize {
    let n = n / 2;

    let mut str = String::from("b");

    for i in 1..=n {
        match i % 3 {
            0 => str = format!("b{}b", str),
            1 => str = format!("a{}c", str),
            2 => str = format!("c{}a", str),
            _ => println!("{}", s),
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

    struct TestCase(usize, String, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, String::from("abc"), 1),
            TestCase(6, String::from("abcabc"), -1),
            TestCase(7, String::from("atcoder"), -1),
            TestCase(19, String::from("bcabcabcabcabcabcab"), 9),
            TestCase(1, String::from("b"), 0),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(expected, run(n, s));
        }
    }
}
