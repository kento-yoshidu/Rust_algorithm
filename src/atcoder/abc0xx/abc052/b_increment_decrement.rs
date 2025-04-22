// https://atcoder.jp/contests/abc052/tasks/abc052_b

fn run(_n: usize, s: &str) -> isize {
    let mut ans = 0;
    let mut tmp = 0;

    s.chars().for_each(|c| {
        match c {
            'I' => {
                tmp += 1;
                ans = ans.max(tmp);
            },
            'D' => {
                tmp -= 1;
            },
            _ => unreachable!(),
        }
    });

    ans
}

fn run2(_n: usize, s: &str) -> isize {
    s.chars()
        .fold((0, 0), |(tmp, max), c| {
            match c {
                'I' => (tmp+1, max.max(tmp+1)),
                'D' => (tmp-1, max),
                _ => unreachable!(),
            }
        }).1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "IIDID", 2),
            TestCase(7, "DDIDDII", 0),
            TestCase(7, "DDDDDDD", 0),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
