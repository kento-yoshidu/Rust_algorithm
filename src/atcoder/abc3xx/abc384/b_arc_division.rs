// https://atcoder.jp/contests/abc384/tasks/abc384_b

fn run(_n: usize, r: isize, da: Vec<(usize, isize)>) -> isize {
    da.into_iter()
        .fold(r, |rate, (d, a)| {
            match d {
                1 => {
                    if 1600 <= rate && rate <= 2799 {
                        rate + a
                    } else {
                        rate
                    }
                },
                2 => {
                    if 1200 <= rate && rate <= 2399 {
                        rate + a
                    } else {
                        rate
                    }
                },
                _ => unreachable!(),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<(usize, isize)>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 1255, vec![(2, 900), (1, 521), (2, 600), (1, 52)], 2728),
            TestCase(2, 3031, vec![(1, 1000), (2, -1000)], 3031),
            TestCase(15, 2352, vec![ (2, -889), (2, 420), (2, -275), (1, 957), (1, -411), (1, -363), (1, 151), (2, -193), (2, 289), (2, -770), (2, 109), (1, 345), (2, 551), (1, -702), (1, 355)], 1226),
        ];

        for TestCase(n, r, da, expected) in tests {
            assert_eq!(run(n, r, da), expected);
        }
    }
}
