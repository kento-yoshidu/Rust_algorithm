// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdout_3

fn run() -> usize {
    813
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize);

    #[test]
    fn paiza_d_level_up_01_step03() {
        let tests = [
            TestCase(813),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
