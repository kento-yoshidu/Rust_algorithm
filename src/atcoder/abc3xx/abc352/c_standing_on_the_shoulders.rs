// https://atcoder.jp/contests/abc349/tasks/abc352_b

pub fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    let pos = ab.iter()
        .enumerate()
        .map(|(i, (a, b))| (i, b - a))
        .max_by_key(|(_, diff)| *diff)
        .unwrap()
        .0;

    let sum: usize = ab.iter()
        .map(|(a, _)| a)
        .sum();

    sum + ab[pos].1 - ab[pos].0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(4, 10), (5, 8), (2, 9)], 18),
            TestCase(5, vec![(1, 1), (1, 1), (1, 1), (1, 1), (1, 1)], 5),
            TestCase(10, vec![(690830957, 868532399), (741145463, 930111470), (612846445, 948344128), (540375785, 925723427), (723092548, 925021315), (928915367, 973970164), (563314352, 832796216), (562681294, 868338948), (923012648, 954764623), (691107436, 891127278)], 7362669937),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
