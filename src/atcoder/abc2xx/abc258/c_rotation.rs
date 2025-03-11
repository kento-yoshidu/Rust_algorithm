// https://atcoder.jp/contests/abc258/tasks/abc258_c

fn run(n: usize, _q: usize, s: &str, query: Vec<(usize, usize)>) -> Vec<char> {
    let vec: Vec<char> = s.chars().collect();

    let mut count = 0;

    let mut ans = Vec::new();

    for (t, x) in query {
        match t {
            1 => {
                count += x;
            },
            2 => {
                println!("2");

                if x < count {
                    count %= n;
                }

                ans.push(vec[(x + n - count - 1) % n]);
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, Vec<(usize, usize)>, Vec<char>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, "abc", vec![(2, 2), (1, 1), (2, 2)], vec!['b', 'a']),
            TestCase(10, 8, "dsuccxulnl", vec![(2, 4), (2, 7), (1, 2), (2, 7), (1, 1), (1, 2), (1, 3), (2, 5)], vec!['c', 'u', 'c', 'u']),
        ];

        for TestCase(n, q, s, query, expected) in tests {
            assert_eq!(run(n, q, s, query), expected);
        }
    }
}
