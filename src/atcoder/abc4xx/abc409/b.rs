//

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    for i in (0..=100).rev() {
        let mut count = 0;

        for num in a.iter() {
            if *num >= i {
                count += 1;
            }
        }

        if count >= i {
            return i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc409_b() {
        let tests = [
            TestCase(2, vec![1, 2, 1], 1),
            TestCase(7, vec![1, 6, 2, 10, 2, 3, 2], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
