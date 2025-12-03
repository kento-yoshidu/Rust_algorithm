// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_string_step1

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .map(|num| num.to_string().len())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn paiza_c_level_up_02_step01() {
        let tests = [
            TestCase(2, vec![10, 100], vec![2, 3]),
            TestCase(3, vec![1234, 0, 99], vec![4, 1, 2]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
