// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__conditions_branch_boss

fn run(n: usize) -> &'static str {
    match n {
        7 => "YES",
        _ => "NO",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn paiza_d_level_up_05_step_final() {
        let tests = [
            TestCase(7, "YES"),
            TestCase(3, "NO"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
