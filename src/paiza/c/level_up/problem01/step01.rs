// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_std_in_out_step1

fn run<'a>(s: &'a str) -> &'a str {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn paiza_c_level_up_01_step01() {
        let tests = [
            TestCase("paiza", "paiza"),
            TestCase("Paiza21", "Paiza21"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
