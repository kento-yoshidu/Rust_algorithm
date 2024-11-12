// https://atcoder.jp/contests/abc025/tasks/abc025_a

fn run(s: &str, n: usize) -> String {
    let vec: Vec<_> = s.chars().collect();

    let mut count = 0;
    let mut ans = String::new();

    for a in &vec {
        for b in &vec {
            count += 1;

            if count == n {
                ans.push(*a);
                ans.push(*b);
            }
        }
    }

    ans
}

fn run2(s: &str, n: usize) -> String {
    let vec: Vec<_> = s.chars().collect();

    let mut ans = String::new();

    ans.push(vec[(n-1) / 5]);
    ans.push(vec[(n-1) % 5]);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("abcde", 8, "bc"),
            TestCase("aeiou", 22, "ue"),
            TestCase("vwxyz", 25, "zz"),
        ];

        for TestCase(s, n, expected) in tests {
            assert_eq!(run(s, n), expected);
        }
    }
}
