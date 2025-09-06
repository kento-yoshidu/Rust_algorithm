// https://paiza.jp/works/mondai/s_rank_level_up_problems/s_rank_level_up_problems__number_theory_2

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;

    base %= m;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % m;
        }

        base = base * base % m;
        exp /= 2;
    }

    result
}

fn run(n: u64, p: u64) -> Vec<u64> {
    let mut ans = Vec::new();

    for i in 1..=n {
        ans.push(mod_pow(i, p-2, p));
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u64, u64, Vec<u64>);

    #[test]
    fn paiza_s_level_up_02_step02() {
        let tests = [
            TestCase(4, 5, vec![1, 3, 2, 4]),
            TestCase(6, 7, vec![1, 4, 5, 2, 3, 6]),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
