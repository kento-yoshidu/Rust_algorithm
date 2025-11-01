// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_std_in_out_boss

fn run<'a>(_n: usize, s: Vec<(&'a str, usize)>) -> Vec<(&'a str, usize)> {
    s.into_iter()
        .map(|(s, a)| (s, a+1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, Vec<(&'static str, usize)>);

    #[test]
    fn paiza_c_level_up_problem01_step_final() {
        let tests = [
            TestCase(1, vec![("Yamada", 30)], vec![("Yamada", 31)]),
            TestCase(3, vec![("Tanaka", 18), ("Sato", 50), ("Suzuki", 120)], vec![("Tanaka", 19), ("Sato", 51), ("Suzuki", 121)]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
