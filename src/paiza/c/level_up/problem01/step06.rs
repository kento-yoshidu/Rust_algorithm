// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_std_in_out_step5

fn run<'a>(_n: usize, a: &'a Vec<usize>) -> &'a Vec<usize> {
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn paiza_c_level_up_problem01_step06() {
        let tests = [
            TestCase(2, vec![1, 5], vec![1, 5]),
            TestCase(3, vec![4, 30, 12], vec![4, 30, 12]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), &expected);
        }
    }
}
