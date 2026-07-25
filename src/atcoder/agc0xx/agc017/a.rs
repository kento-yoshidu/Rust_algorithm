// https://atcoder.jp/contests/agc017/tasks/agc017_a

fn run(n: usize, p: usize, a: Vec<usize>) -> usize {
    let even_count = a.into_iter().filter(|n| *n % 2 == 0).count();
    let odd_count = n - even_count;

    if odd_count == 0 {
        match p {
            0 => 2usize.pow(n as u32),
            1 => 0,
            _ => unreachable!(),
        }
    } else {
        2usize.pow(even_count as u32) * 2usize.pow((odd_count - 1) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn agc017_a() {
        let tests = [
            TestCase(2, 0, vec![1, 3], 2),
            TestCase(1, 1, vec![50], 0),
            TestCase(3, 0, vec![1, 1, 1], 4),
            TestCase(45, 1, vec![17, 55, 85, 55, 74, 20, 90, 67, 40, 70, 39, 89, 91, 50, 16, 24, 14, 43, 24, 66, 25, 9, 89, 71, 41, 16, 53, 13, 61, 15, 85, 72, 62, 67, 42, 26, 36, 66, 4, 87, 59, 91, 4, 25, 26], 17592186044416),
        ];

        for TestCase(n, p, a, expected) in tests {
            assert_eq!(run(n, p, a), expected);
        }
    }
}
