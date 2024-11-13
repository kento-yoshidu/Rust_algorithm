// https://atcoder.jp/contests/abc022/tasks/abc022_a

fn run(_n: usize, s: isize, t: isize, w: Vec<isize>) -> usize {
    let vec = w.into_iter()
        .scan(0, |state, x| {
            *state = *state + x;
            Some(*state)
        }).collect::<Vec::<isize>>();

    vec.into_iter()
        .filter(|num| {
            s <= *num && *num <= t
        }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, isize, Vec<isize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 60, 70, vec![50, 10, 10, 10, 10], 2),
            TestCase(5, 50, 100, vec![120, -10, -20, -30, 70], 2),
            TestCase(5, 50, 100, vec![120, -1, -1, -1, -1], 0),
            TestCase(5, 50, 100, vec![100, -1, -1, -1, -1], 5),
        ];

        for TestCase(n, s, t, w, expected) in tests {
            assert_eq!(expected, run(n, s, t, w));
        }
    }
}
