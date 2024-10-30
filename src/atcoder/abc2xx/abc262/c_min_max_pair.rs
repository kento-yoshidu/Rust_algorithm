// https://atcoder.jp/contests/abc262/tasks/abc262_c

pub fn run(n: usize, a: Vec<usize>) -> usize {
    let same = a.iter()
        .enumerate()
        .filter(|(i, n)| {
            i+1 == **n
        })
        .count();

    let mut ans = same * (same-1) / 2;

    for i in 0..n {
        if i + 1 < a[i] && i + 1 == a[a[i]-1] {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![1, 3, 2, 4], 2),
            TestCase(10, vec![5, 8, 2, 2, 1, 6, 7, 2, 9, 10], 8),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }

    }
}
