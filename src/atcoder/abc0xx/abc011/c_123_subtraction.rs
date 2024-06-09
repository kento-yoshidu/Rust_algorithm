// https://atcoder.jp/contests/abc011/tasks/abc011_3

pub fn run(n: isize, ng: [isize; 3]) -> &'static str {
    let mut cur = n;

    for _ in 0..100 {
        if cur == 0 {
            return "YES";
        }

        if cur < 0 {
            return "NO";
        }
        if cur >= 3 && !ng.contains(&(cur - 3)) {
            cur -= 3;
            continue;
        }

        if cur >= 2 && !ng.contains(&(cur - 2)) {
            cur -= 2;
            continue;
        }

        if cur >= 1 && !ng.contains(&(cur - 1)) {
            cur -= 1;
            continue;
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
