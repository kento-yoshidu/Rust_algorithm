// https://atcoder.jp/contests/abc180/tasks/abc180_c

fn run(n: usize) -> Vec<usize> {
    let mut ans = Vec::new();

    for i in 1..=(n as f64).sqrt() as usize {
        if n % i == 0 {
            let j = n / i;

            ans.push(i);

            if i != j {
                ans.push(j);
            }
        }
    }

    ans.sort();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn abc180_c() {
        let tests = [
            TestCase(6, vec![1, 2, 3, 6]),
            TestCase(720, vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 30, 36, 40, 45, 48, 60, 72, 80, 90, 120, 144, 180, 240, 360, 720]),
            TestCase(1000000007, vec![1, 1000000007]),
            TestCase(1, vec![1]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
