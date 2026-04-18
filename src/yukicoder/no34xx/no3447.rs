// https://yukicoder.me/problems/no/3447

fn run(n: usize, a: usize, b: usize) -> Vec<usize> {
    (0..n)
        .map(|num| a * (b.pow(num as u32)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>);

    #[test]
    fn yuki_3447() {
        let tests = [
            TestCase(1, 6, 7, vec![6]),
            TestCase(9, 2, 4, vec![2, 8, 32, 128, 512, 2048, 8192, 32768, 131072]),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
