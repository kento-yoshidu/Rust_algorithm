// https://atcoder.jp/contests/abc381/tasks/abc381_c

use crate::atcoder::abc0xx::abc054::b_template_matching;

fn run(n: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut ans = 0;

    for i in 0..n {
        if chars[i] == '/' {
            let mut dis = 0;

            loop {
                if dis >= i || i+dis >= n {
                    break;
                }

                if chars[i-dis-1] == '1' && chars[i+dis+1] == '2' {
                    dis += 1;
                } else {
                    break
                }

            }

            ans = ans.max(dis);
        }
    }

    ans * 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, "211/2212", 5),
            TestCase(5, "22/11", 1),
            TestCase(22, "/1211/2///2111/2222/11", 7),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected)
        }
    }
}
