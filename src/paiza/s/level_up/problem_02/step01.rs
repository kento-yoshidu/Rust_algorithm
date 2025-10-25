// https://paiza.jp/works/mondai/s_rank_level_up_problems/s_rank_level_up_problems__number_theory_1

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

fn run(x: u64, p: u64) -> u64 {
    mod_pow(x, p - 2, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u64, u64, u64);

    #[test]
    fn paiza_s_level_up_02_step01() {
        let tests = [
            TestCase(1, 3, 1),
            TestCase(2, 1000000007, 500000004),
        ];

        for TestCase(x, p, expected) in tests {
            assert_eq!(run(x, p), expected);
        }
    }
}
