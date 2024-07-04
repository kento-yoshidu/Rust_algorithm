// https://atcoder.jp/contests/nikkei2019-qual/tasks/nikkei2019_qual_b

pub fn run(n: usize, a: &str, b: &str, c: &str) -> usize {
    let mut ans = 0;

    for i in 0..n {
        let ac = a.chars().nth(i).unwrap();
        let bc = b.chars().nth(i).unwrap();
        let cc = c.chars().nth(i).unwrap();

        if ac == bc && bc == cc {
            continue;
        } else if ac == bc || bc == cc || ac == cc {
            ans += 1;
        } else {
            ans += 2;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "west", "east", "wait", 3),
            TestCase(9, "different", "different", "different", 0),
            TestCase(7, "zenkoku", "touitsu", "program", 13),
        ];

        for TestCase(n, a, b, c, expected) in tests {
            assert_eq!(run(n, a, b, c), expected);
        }
    }
}
