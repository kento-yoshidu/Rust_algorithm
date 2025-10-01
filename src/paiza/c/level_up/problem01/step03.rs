// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_std_in_out_step3

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn paiza_c_level_up_problem01_step03() {
        let tests = [
            TestCase(2, vec![1, 2], vec![1, 2]),
            TestCase(3, vec![10, 5, 39], vec![10, 5, 39]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
