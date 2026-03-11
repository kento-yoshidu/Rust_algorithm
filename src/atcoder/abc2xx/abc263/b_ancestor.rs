// https://atcoder.jp/contests/abc263/tasks/abc263_b

fn run(n: usize, p: &Vec<usize>) -> usize {
    let mut ans = 1;
    let mut k = n-1;

    loop {
        if p[k-1] == 1 {
            return ans;
        }

        k = p[k-1]-1;
        ans += 1;
    }
}

fn calc(p: &Vec<usize>, count: usize, k: usize) -> usize {
    if p[k-1] == 1 {
        return count
    } else {
        let k = p[k-1]-1;
        calc(p, count + 1, k)
    }
}

fn run2(n: usize, p: &Vec<usize>) -> usize {
    calc(p, 1, n-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc263_a() {
        let tests = [
            TestCase(3, vec![1, 2], 2),
            TestCase(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 9),
            TestCase(26, vec![1, 2, 3, 2, 1, 1, 1, 8, 4, 7, 6, 1, 3, 3, 2, 6, 17, 13, 6, 15, 20, 2, 8, 24, 17], 3),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, &p), expected);
        }
    }
}
