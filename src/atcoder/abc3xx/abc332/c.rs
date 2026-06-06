// https://atcoder.jp/contests/abc332/tasks/abc332_c

fn run(_n: usize, m: usize, s: &str) -> usize {
    let vec: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

    vec.into_iter()
        .fold((0, (m, 0)), |(ans, (muji, logo)), num| {
            match num {
                0 => {
                    (ans, (m, ans))
                },
                1 => {
                    if muji > 0 {
                        (ans, (muji-1, logo))
                    } else if logo > 0 {
                        (ans, (muji, logo-1))
                    } else {
                        (ans+1, (muji, logo))
                    }
                },
                2 => {
                    if logo > 0 {
                        (ans, (muji, logo-1))
                    } else {
                        (ans+1, (muji, logo))
                    }
                },
                _ => unreachable!(),
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, usize);

    #[test]
    fn abc332_c() {
        let tests = [
            TestCase(6, 1, "112022", 2),
            TestCase(3, 1, "222", 3),
            TestCase(2, 1, "01", 0),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, s), expected);
        }
    }
}
