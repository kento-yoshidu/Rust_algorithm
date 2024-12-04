// https://atcoder.jp/contests/abc042/tasks/arc058_a

fn run(n: usize, _k: usize, d: Vec<usize>) -> usize {
    let chars: Vec<char> = d.iter()
        .map(|num| char::from_digit(*num as u32, 10).unwrap())
        .collect();

    (n..=n*10)
        .find(|num| {
            !num.to_string().contains(&*chars)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1000, 8, vec![1, 3, 4, 5, 6, 7, 8, 9], 2000),
            TestCase(9999, 1, vec![0], 9999),
        ];

        for TestCase(n, k, d, expected) in tests {
            assert_eq!(run(n, k, d), expected);
        }
    }
}
