// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/5/ITP1_5_D

fn has_digit_3(mut x: usize) -> bool {
    while x > 0 {
        if x % 10 == 3 {
            return true;
        }
        x /= 10;
    }

    false
}

fn run(n: usize) -> Vec<usize> {
    (1..=n)
        .filter(|num| {
            num % 3 == 0 || has_digit_3(*num)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn itp1_5_d() {
        let tests = [
            TestCase(30, vec![3, 6, 9, 12, 13, 15, 18, 21, 23, 24, 27, 30]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
