// https://paiza.jp/works/mondai/s_rank_level_up_problems/s_rank_level_up_problems__number_theory_3

fn run(n: u64, p: u64) -> Vec<u64> {
    let mut ans = Vec::new();

    let mut cur = 1;

    for x in 1..=n {
        cur = (cur * x) % p;
        ans.push(cur);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u64, u64, Vec<u64>);

    #[test]
    fn paiza_s_level_up_02_step03() {
        let tests = [
            TestCase(4, 5, vec![1, 2, 1, 4]),
            TestCase(6, 7, vec![1, 2, 6, 3, 1, 6]),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
