// https://atcoder.jp/contests/abc315/tasks/abc315_b

fn run(_m: usize, d: &Vec<isize>) -> (usize, usize) {
    let mut middle = d.iter().sum::<isize>() / 2 + 1;

    for (index, d) in d.into_iter().enumerate() {
        if middle - d <= 0 {
            return (index+1, middle as usize)
        }

        middle -= d;
    }

    unreachable!();
}

fn calc(rest: isize, i: usize, d: &Vec<isize>) -> (usize, usize) {
    if rest - d[i] <= 0 {
        return (i + 1, rest as usize);
    }

    calc(rest - d[i], i + 1, d)
}

fn run2(_m: usize, d: &Vec<isize>) -> (usize, usize) {
    let rest = d.iter().sum::<isize>() / 2 + 1;

    calc(rest, 0, &d)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, (usize, usize));

    #[test]
    fn abc315_b() {
        let tests = [
            TestCase(12, vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], (7, 2)),
            TestCase(1, vec![1], (1, 1)),
            TestCase(6, vec![3, 1, 4, 1, 5, 9], (5, 3)),
        ];

        for TestCase(m, d, expected) in tests {
            assert_eq!(run(m, &d), expected);
            assert_eq!(run2(m, &d), expected);
        }
    }
}
