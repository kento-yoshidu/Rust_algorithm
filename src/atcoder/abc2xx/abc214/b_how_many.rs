// https://atcoder.jp/contests/abc214/tasks/abc214_b

fn run(s: usize, t: usize) -> usize{
    let mut count = 0;

    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a*b*c > t {
                    break
                }
                if a+b+c > s {
                    break
                }

                if a*b*c <= t {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 0, 1),
            TestCase(1, 0, 4),
            TestCase(2, 5, 10),
            TestCase(10, 10, 213),
            TestCase(30, 100, 2471),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
