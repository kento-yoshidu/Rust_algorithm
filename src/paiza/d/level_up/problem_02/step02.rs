// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__arithmetic_substitution_op_2

fn run() -> (usize, usize) {
    (437326 / 9085, 437326 % 9085)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase((usize, usize));

    #[test]
    fn paiza_d_level_up_02_step02() {
        let tests = [
            TestCase((48, 1246)),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
