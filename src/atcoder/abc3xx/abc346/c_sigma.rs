// https://atcoder.jp/contests/abc346/tasks/abc346_c

pub fn run(_n: usize, k: usize, a: Vec<usize>) -> usize {
    let mut total = k * (k + 1) / 2;

    let mut vec = a.clone();
    vec.sort();
    vec.dedup();

    for num in vec {
        if num > k {
            break;
        }

        total -= num;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 5, vec![1, 6, 3, 1], 11),
            TestCase(1, 3, vec![346], 6),
            TestCase(10, 158260522, vec![877914575, 24979445, 623690081, 262703497, 24979445, 1822804784, 1430302156, 1161735902, 923078537, 1189330739], 12523196466007058),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
