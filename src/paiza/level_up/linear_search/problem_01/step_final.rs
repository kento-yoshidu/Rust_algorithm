// https://paiza.jp/works/mondai/sequence_search_problems/sequence_search_problems_search_value_boss

fn run(_n: usize, a: Vec<isize>, k: isize) -> Vec<usize> {
    a.into_iter()
        .enumerate()
        .filter(|(_, num)| *num == k)
        .map(|(i, _)| i+1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize, Vec<usize>);

    #[test]
    fn paiza_level_up_linear_search_problem01_step_final() {
        let tests = [
            TestCase(5, vec![-3, 2, 0, -1, 2], 2, vec![2, 5]),
        ];

        for TestCase(n, a, k, expected) in tests {
            assert_eq!(run(n, a, k), expected);
        }
    }
}
