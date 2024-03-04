// https://atcoder.jp/contests/tenka1-2018-beginner/tasks/tenka1_2018_b

fn run(a: usize, b: usize, k: usize) -> (usize, usize) {
    let mut aa = a;
    let mut bb = b;

    for i in 0..k {
        if i % 2 == 0 {
            if aa % 2 != 0 {
                aa -= 1;
            }

            bb += aa / 2;
            aa /= 2;
        } else {
            if bb % 2 != 0 {
                bb -= 1;
            }

            aa += bb / 2;
            bb /= 2;
        }
    }

    (aa, bb)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 4, 2, (5, 3)),
            TestCase(3, 3, 3, (1, 3)),
            TestCase(314159265, 358979323, 84, (448759046, 224379523)),
        ];

        for TestCase(a, b, k, expected) in tests {
            assert_eq!(run(a, b, k), expected);
        }
    }
}
