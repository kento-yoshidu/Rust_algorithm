// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdout_2

fn run() -> &'static str {
    "paiza learning"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str);

    #[test]
    fn paiza_d_level_up_01_step02() {
        let tests = [
            TestCase("paiza learning"),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
