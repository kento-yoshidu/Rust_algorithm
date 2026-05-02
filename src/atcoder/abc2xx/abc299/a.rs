// https://atcoder.jp/contests/abc299/tasks/abc299_a

fn run(_n: usize, s: &str) -> &'static str {
    let l = s.find('|').unwrap();
    let r = s.rfind('|').unwrap();
    let o = s.find('*').unwrap();

    if l < o && o < r {
        "in"
    } else {
        "out"
    }
}

fn run2(_n: usize, s: &str) -> &'static str {
    let mut l = -1;
    let mut m = -1;
    let mut r = -1;

    for (i, c) in s.chars().enumerate() {
        if c == '|' {
            if l == -1 {
                l = i as i32;
            } else {
                r = i as i32;
            }
        }

        if c == '*' {
            m = i as i32;
        }
    }

    if l < m && m < r {
        "in"
    } else {
        "out"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc299_a() {
        let tests = [
            TestCase(10, ".|..*...|.", "in"),
            TestCase(10, ".|..|.*...", "out"),
            TestCase(3, "|*|", "in"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
