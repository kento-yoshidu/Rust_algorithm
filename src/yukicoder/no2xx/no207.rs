// https://yukicoder.me/problems/no/207

fn run(a: usize, b: usize) -> Vec<usize> {
    (a..=b)
        .filter(|num| {
            let str = num.to_string();

            str.chars().any(|c| c == '3') || num % 3 == 0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>);

    #[test]
    fn yuki_207() {
        let tests = [
            TestCase(1, 10, vec![3, 6, 9]),
            TestCase(10, 20, vec![12, 13, 15, 18]),
            TestCase(30, 40, vec![30, 31, 32, 33, 34, 35, 36, 37, 38, 39]),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
