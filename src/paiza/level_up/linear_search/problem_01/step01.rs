// https://paiza.jp/works/mondai/sequence_search_problems/sequence_search_problems_search_value_step0

fn run(_n: usize, a: Vec<isize>, k: isize) -> usize {
    a.into_iter()
        .filter(|num| k == *num)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize, usize);

    #[test]
    fn paiza_level_up_linear_search_problem01_step01() {
        let tests = [
            TestCase(5, vec![-3, 2, 0, -1, 2], 2, 2),
        ];

        for TestCase(n, a, k, expected) in tests {
            assert_eq!(run(n, a, k), expected);
        }
    }
}
