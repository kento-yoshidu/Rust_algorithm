// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__accompanied_by_stdin_3

fn run(a: usize, b: usize, c: usize) -> usize {
    a * b % c
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn paiza_d_level_up_04_step03() {
        let tests = [
            TestCase(149, 825, 262, 47),
            TestCase(581, 938, 515, 108),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
