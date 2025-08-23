// https://paiza.jp/works/mondai/s_rank_level_up_problems/s_rank_level_up_problems__game_theory_1

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .fold(0, |acc, n| {
            acc ^ n
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn paiza_s_level_up_01_step01() {
        let tests = [
            TestCase(5, vec![1, 2, 3, 4, 5], 1),
            TestCase(2, vec![63, 31], 32),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
