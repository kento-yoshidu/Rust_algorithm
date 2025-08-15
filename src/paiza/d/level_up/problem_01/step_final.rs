// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdout_boss

fn run() -> &'static str {
    "1 1"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str);

    #[test]
    fn paiza_d_level_up_01_step_final() {
        let tests = [
            TestCase("1 1"),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
