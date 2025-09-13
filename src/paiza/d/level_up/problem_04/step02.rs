// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__accompanied_by_stdin_2

fn run(a: isize, b: isize) -> (isize, isize) {
    (a - b, a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, (isize, isize));

    #[test]
    fn paiza_d_level_up_04_step02() {
        let tests = [
            TestCase(397, 646, (-249, 256462)),
            TestCase(343, 444, (-101, 152292)),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
