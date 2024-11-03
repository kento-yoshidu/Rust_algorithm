// https://atcoder.jp/contests/abc378/tasks/abc378_a

fn run(a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let mut vec = [0; 4];

    for i in [a1, a2, a3, a4] {
        vec[i-1] += 1;
    }

    vec.into_iter()
        .map(|i| {
            if i == 4 {
                2
            } else if i > 1 {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 1, 2, 1, 2),
            TestCase(4, 4, 4, 1, 1),
            TestCase(1, 2, 3, 4, 0),
            TestCase(4, 4, 4, 4, 2),
        ];

        for TestCase(a1, a2, a3, a4, expected) in tests {
            assert_eq!(run(a1, a2, a3, a4), expected);
        }
    }
}
