// https://yukicoder.me/problems/no/211

fn run(k: usize) -> f64 {
    let mut count = 0;

    for i in [2, 3, 5, 7, 11, 13] {
        for j in [4, 6, 8, 9, 10, 12] {
            if i * j == k {
                count += 1;
            }
        }
    }

    count as f64 / 36.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn yuki_211() {
        let tests = [
            TestCase(7, 0.0),
            TestCase(50, 0.027777777777777776),
        ];

        for TestCase(k, expected) in tests {
            assert_eq!(run(k), expected);
        }
    }
}
