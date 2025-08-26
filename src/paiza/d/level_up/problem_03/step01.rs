// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdin_1

fn run<'a>(s: &'a str) -> &'a str {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn paiza_d_level_up_03_step01() {
        let tests = [
            TestCase("paiza", "paiza"),
            TestCase("Paiza21", "Paiza21"),
            TestCase("paiza813", "paiza813"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
