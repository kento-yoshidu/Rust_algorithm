// https://atcoder.jp/contests/abc115/tasks/abc115_b

fn run(_n: usize, p: Vec<usize>) -> usize {
    let mut vec: Vec<usize> = p.clone();

    vec.sort();
    vec.reverse();

    vec.iter()
        .enumerate()
        .map(|(i, price)| {
            if i == 0 {
                price / 2
            } else {
                *price
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![4980, 7980, 6980], 15950),
            TestCase(4, vec![4320, 4320, 4320, 4320], 15120),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, q), expected);
        }
    }
}
