// https://yukicoder.me/problems/no/2938

fn run(n: usize, a: Vec<isize>) -> isize {
    let mut ans = 0;

    for i in 0..n {
        for j in 0..n {
            ans += (a[i] - a[j]).abs() * (i as isize - j as isize).abs();
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn yuki_2938() {
        let tests = [
            TestCase(2, vec![3, 2], 2),
            TestCase(3, vec![1, 2, 3], 12),
            TestCase(9, vec![41, 15, 76, 69, 70, 58, 10, 20, 34], 7196),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
