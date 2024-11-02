// https://atcoder.jp/contests/abc369/tasks/abc369_b

fn run(_n: usize, a: Vec<(isize, char)>) -> usize {
    let (ans, _) = a.into_iter()
        .fold((0, (0, 0)), |(mut ans, mut pos), (a, s)| {
            match s {
                'L' => {
                    if pos.0 == 0 {
                        pos.0 = a;
                    } else {
                        ans += (pos.0 - a).abs();
                        pos.0 = a;
                    }

                    (ans, pos)
                },
                'R' => {
                    if pos.1 == 0 {
                        pos.1 = a;
                    } else {
                        ans += (pos.1 - a).abs();
                        pos.1 = a;
                    }

                    (ans, pos)
                },
                _ => unreachable!(),
            }
        });

    ans as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, char)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(3, 'L'), (6, 'R'), (9, 'L'), (1, 'R')], 11),
            TestCase(3, vec![(2, 'L'), (2, 'L'), (100, 'L')], 98),
            TestCase(8, vec![(22, 'L'), (75, 'L'), (26, 'R'), (45, 'R'), (72, 'R'), (81, 'R'), (47, 'L'), (29, 'R')], 188),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
