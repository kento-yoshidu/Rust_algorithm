// https://atcoder.jp/contests/abc200/tasks/abc200_c

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut vec: Vec<isize> = vec![0; 200];

    for n in a {
        vec[n%200] += 1;
    }

    (0..200)
        .map(|i| {
            (vec[i] * (vec[i]-1)) / 2
        })
        .sum::<isize>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc200_c() {
        let tests = [
            TestCase(6, vec![123, 223, 123, 523, 200, 2000], 4),
            TestCase(5, vec![1, 2, 3, 4, 5], 0),
            TestCase(8, vec![199, 100, 200, 400, 300, 500, 600, 200], 9),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
