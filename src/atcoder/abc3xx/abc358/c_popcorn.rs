// https://atcoder.jp/contests/abc358/tasks/abc358_c

pub fn run(n: usize, m: usize, s: Vec<&str>) -> usize {
    let mut ans = n;

    for bit in 0..1 << n {
        let mut flag = vec![false; m];
        let mut count = 0;

        for i in 0..n {
            if bit >> i & 1 == 1 {
                for (i, c) in s[i].chars().enumerate() {
                    if c == 'o' {
                        flag[i] = true;
                    }
                }

                count += 1;
            }
        }

        if flag.into_iter().all(|f| f == true) {
            ans = ans.min(count);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, vec!["oooxx", "xooox", "xxooo"], 2),
            TestCase(3, 2, vec!["oo", "ox", "xo"], 1),
            TestCase(8, 6, vec!["xxoxxo", "xxoxxx", "xoxxxx", "xxxoxx", "xxoooo", "xxxxox", "xoxxox", "oxoxxo"], 3),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, s), expected);
        }
    }
}
