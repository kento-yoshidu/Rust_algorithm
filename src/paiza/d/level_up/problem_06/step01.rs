// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__loop_1

fn run(n: usize) -> Vec<usize> {
    (1..=n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn paiza_d_level_up_06_step01() {
        let tests = [
            TestCase(1, vec![1]),
            TestCase(5, vec![1, 2, 3, 4, 5]),
            TestCase(28, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
