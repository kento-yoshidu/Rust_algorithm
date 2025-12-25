// https://atcoder.jp/contests/abc216/tasks/abc216_c

fn run(n: isize) -> String {
    let mut ans = String::new();

    let mut num = n;

    loop {
        if num == 0 {
            break;
        } else if num % 2 == 0 {
            num /= 2;
            ans.push('B');
        } else {
            num -= 1;
            ans.push('A');
        }
    }

    ans.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, &'static str);

    #[test]
    fn abc216_c() {
        let tests = [
            TestCase(5, "ABBA"),
            TestCase(14, "ABABAB"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
