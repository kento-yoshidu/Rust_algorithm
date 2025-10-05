// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_std_in_out_step4

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn paiza_c_level_up_problem01_step04() {
        let tests = [
            TestCase(2, vec![4, 7], 7),
            TestCase(3, vec![20, 19, 2], 20),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
