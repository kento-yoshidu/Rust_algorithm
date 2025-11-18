// https://atcoder.jp/contests/abc156/tasks/abc156_c

fn run(_n: usize, vec: &Vec<isize>) -> isize {
    let mut ans = 10000;

    for i in 0..=100 {
        let mut sum = 0;

        for v in vec.iter() {
            sum += (v - (i+1) as isize).pow(2)
        }

        ans = ans.min(sum);
    }

    ans
}

fn run2(_n: usize, x: &Vec<isize>) -> isize {
    (0..=100)
        .map(|i| {
            x.into_iter()
                .fold(0, |state, x| {
                    state + (x - i+1).pow(2)
                })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc156_c() {
        let tests = [
            TestCase(2, vec![1, 4], 5),
            TestCase(7, vec![14, 14, 2, 13, 56, 2, 37], 2354),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, &x), expected);
            assert_eq!(run2(n, &x), expected);
        }
    }
}
