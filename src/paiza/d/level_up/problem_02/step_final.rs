// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__arithmetic_substitution_op_boss

fn run() -> usize {
    3286 * 4736 % 12312
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize);

    #[test]
    fn paiza_d_level_up_02_step_final() {
        let tests = [
            TestCase(128),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
