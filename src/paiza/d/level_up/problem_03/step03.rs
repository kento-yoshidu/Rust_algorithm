// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdin_3

fn run<'a>(s: &'a str) -> &'a str {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn paiza_d_level_up_03_step03() {
        let tests = [
            TestCase("hello", "hello"),
            TestCase("8132020", "8132020"),
            TestCase("paiza813", "paiza813"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
