// https://atcoder.jp/contests/abc063/tasks/arc075_a

pub fn run(_n: usize, s: Vec<usize>) -> usize {
    if s.iter()
        .all(|num| num % 10 == 0) {
            return 0
        }

    let sum = s.iter().sum();

    if sum % 10 != 0 {
        return sum
    }

    let mut vec = s.clone();
    vec.sort();

    for i in vec {
        if (sum - i) % 10 != 0 {
            return sum - i
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![5, 10, 15], 25),
            TestCase(3, vec![10, 10, 15], 35),
            TestCase(3, vec![10, 20, 30], 0),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(expected, run(n, s));
        }
    }
}
