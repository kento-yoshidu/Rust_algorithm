// https://atcoder.jp/contests/abc313/tasks/abc313_b

fn run(n: usize, _m: usize, a: Vec<(usize, usize)>) -> isize {
    let mut arr = vec![true; n];

    for v in a {
        arr[(v.1)-1] = false
    }

    if (arr.iter().filter(|num| {
        **num
    }).count()) > 1 {
        -1
    } else {
        arr.iter().position(|num| *num).unwrap() as isize + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, isize);

    #[test]
    fn abc313_b() {
        let tests = [
            TestCase(3, 2, vec![(1, 2), (2, 3)], 1),
            TestCase(3, 2, vec![(1, 3), (2, 3)], -1),
            TestCase(6, 6, vec![(1, 6), (6, 5), (6, 2), (2, 3), (4, 3), (4, 2)], -1),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
