// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__arithmetic_substitution_op_1

fn run() -> usize {
    1231 + 5178
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize);

    #[test]
    fn paiza_d_level_up_02_step01() {
        let tests = [
            TestCase(6409),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
