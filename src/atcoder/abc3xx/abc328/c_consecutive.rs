// https://atcoder.jp/contests/abc328/tasks/abc328_c

fn run(_n: usize, _q: usize, s: &str, vec: Vec<(usize, usize)>) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut cum = vec![0];

    for (i, a) in chars.windows(2).enumerate() {
        if a[0] == a[1] {
            cum.push(cum[i]+1)
        } else {
            cum.push(cum[i])
        }
    }

    vec.into_iter()
        .map(|(l, r)| {
            cum[r-1] - cum[l-1]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(11, 4, "mississippi", vec![(3, 9), (4, 10), (4, 6), (7, 7)], vec![2, 2, 0, 0]),
            TestCase(5, 1, "aaaaa", vec![(1, 5)], vec![4]),
        ];

        for TestCase(n, q, s, vec, expected) in tests {
            assert_eq!(run(n, q, s, vec), expected);
        }
    }
}
