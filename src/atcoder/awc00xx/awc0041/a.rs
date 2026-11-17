// https://atcoder.jp/contests/awc0041/tasks/awc0041_a

fn run(n: usize, _m: usize, ck: Vec<(usize, usize)>, p: Vec<usize>) -> usize {
    let mut count = vec![0; n];

    let mut ans = 0;

    for p in p {
        if count[p-1] < ck[p-1].1 {
            count[p-1] += 1;
            ans += ck[p-1].0;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>, usize);

    #[test]
    fn awc0041_a() {
        let tests = [
            TestCase(2, 5, vec![(500, 2), (1000, 1)], vec![1, 1, 1, 2, 2], 2000),
        ];

        for TestCase(n, m, ck, p, expected) in tests {
            assert_eq!(run(n, m, ck, p), expected);
        }
    }
}
