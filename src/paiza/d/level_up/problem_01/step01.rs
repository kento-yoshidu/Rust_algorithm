// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdout_1

fn run() -> &'static str {
    "paiza"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str);

    #[test]
    fn paiza_d_level_up_01_step01() {
        let tests = [
            TestCase("paiza"),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
