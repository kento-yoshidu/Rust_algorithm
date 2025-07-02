// https://atcoder.jp/contests/abc397/tasks/abc397_b

fn run(s: &str) -> usize {
    let mut ans = 0;

    for i in 0..s.len()-1 {
        if s.chars().nth(i).unwrap() == s.chars().nth(i+1).unwrap() {
            ans += 1;
        }
    }

    if s.chars().nth(0).unwrap() == 'o' {
        ans += 1;
    }

    if s.chars().last().unwrap() == 'i' {
        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("ioi", 1),
            TestCase("iioo", 2),
            TestCase("io", 0),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
