// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdout_4

fn run() -> &'static str {
    "8\n1\n3"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str);

    #[test]
    fn paiza_d_level_up_01_step04() {
        let tests = [
            TestCase("8\n1\n3"),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}
