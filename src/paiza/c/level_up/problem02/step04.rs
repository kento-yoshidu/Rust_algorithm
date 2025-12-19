// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_string_step4

fn run(n: usize) -> String {
    format!("{:03}", n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn paiza_c_level_up_02_step04() {
        let tests = [
            TestCase(7, "007"),
            TestCase(123, "123"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
