// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_std_in_out_step2

fn run(n: usize) -> Vec<&'static str> {
    vec!["paiza"; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>);

    #[test]
    fn paiza_c_level_up_01_step02() {
        let tests = [
            TestCase(2, vec!["paiza", "paiza"]),
            TestCase(5, vec!["paiza", "paiza", "paiza", "paiza", "paiza"]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
