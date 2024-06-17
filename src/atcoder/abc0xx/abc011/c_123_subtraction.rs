// https://atcoder.jp/contests/abc011/tasks/abc011_3

pub fn run(n: isize, ng: [isize; 3]) -> &'static str {
    if ng.contains(&n) {
        return "NO";
    }

    let mut cur = n;

    for _ in 0..100 {
        for j in (1..=3).rev() {
            if ng.contains(&(cur - j)) {
                continue;
            }

            cur -= j;
            break;
        }

        if cur <= 0 {
            return "YES";
        }
    }

    "NO"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, [isize; 3], &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, [1, 7, 15], "YES"),
            TestCase(5, [1, 4, 2], "YES"),
            TestCase(300, [57, 121, 244], "NO"),
        ];

        for TestCase(n, ng, expected) in tests {
            assert_eq!(run(n, ng), expected);
        }
    }
}
