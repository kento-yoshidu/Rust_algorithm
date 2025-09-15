// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__accompanied_by_stdin_boss

fn run(a: usize, b: usize, c: usize) -> usize {
    a - b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn paiza_d_level_up_04_step_final() {
        let tests = [
            TestCase(5, 3, 2, 4),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
