// https://atcoder.jp/contests/abc376/tasks/abc376_b

pub fn run(n: usize, _q: usize, a: Vec<(char, usize)>) -> usize {
    let mut ans = 0;

    let mut l = 1;
    let mut r = 2;

    for (h, t) in a.into_iter() {
        match h {
            'L' => {
                ans += if l < t {
                    if l < r && r < t {
                        n + l - t
                    } else {
                        t - l
                    }
                } else {
                    if t < r && r < l  {
                        n + t - l
                    } else {
                        l - t
                    }
                };

                l = t;
            },
            'R' => {
                ans += if r < t {
                    if r < l && l < t {
                        n + r - t
                    } else {
                        t - r
                    }
                } else {
                    if t < l && l < r {
                        n + t - r
                    } else {
                        r - t
                    }
                };

                r = t;
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(char, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 3, vec![('R', 4), ('L', 5), ('R', 6)], 8),
            TestCase(100, 2, vec![('L', 1), ('R', 2)], 0),
            TestCase(30, 8, vec![('R', 23), ('R', 26), ('R', 29), ('L', 20), ('R', 29), ('R', 19), ('L', 7), ('L', 16)], 92),
        ];

        for TestCase(n, q, a, expected) in tests {
            assert_eq!(run(n, q, a), expected);
        }
    }
}
