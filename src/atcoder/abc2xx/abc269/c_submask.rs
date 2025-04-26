// https://atcoder.jp/contests/abc269/tasks/abc269_c

fn run(n: usize) -> Vec<usize> {
    let mut arr = Vec::new();

    for i in 0..60 {
        if n & (1 << i) != 0 {
            arr.push(i);
        }
    }

    let mut ans = Vec::new();

    for bit in 0..(1 << arr.len()) {
        let mut cur = 0;

        for i in 0..arr.len() {
            if bit & (1 << i) != 0 {
                cur += 1 << arr[i];
            }
        }

        ans.push(cur);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(11, vec![0, 1, 2, 3, 8, 9, 10, 11 ]),
            TestCase(0, vec![0]),
            TestCase(576461302059761664, vec![ 0, 524288, 549755813888, 549756338176, 576460752303423488, 576460752303947776, 576461302059237376, 576461302059761664]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
