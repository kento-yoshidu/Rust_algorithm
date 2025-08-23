// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__arithmetic_substitution_op_3

fn run() -> usize {
    ((202_usize + 134) * 107).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize);

    #[test]
    fn paiza_d_level_up_02_step03() {
        let tests = [
            TestCase(1292546304),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
