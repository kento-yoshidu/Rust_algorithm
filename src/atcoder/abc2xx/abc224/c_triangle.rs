// https://atcoder.jp/contests/abc224/tasks/abc224_c

fn run(n: usize, xy: Vec<(isize, isize)>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if (xy[j].0 - xy[i].0)*(xy[k].1 - xy[i].1) - (xy[k].0 - xy[i].0)*(xy[j].1 - xy[i].1) != 0 {
                    ans += 1;
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(0, 1), (1, 3), (1, 1), (-1, -1)], 3),
            TestCase(20, vec![(224, 433), (987654321, 987654321), (2, 0), (6, 4), (314159265, 358979323), (0, 0), (-123456789, 123456789), (-1000000000, 1000000000), (124, 233), (9, -6), (-4, 0), (9, 5), (-7, 3), (333333333, -333333333), (-9, -1), (7, -10), (-1, 5), (324, 633), (1000000000, -1000000000), (20, 0)], 1124),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
