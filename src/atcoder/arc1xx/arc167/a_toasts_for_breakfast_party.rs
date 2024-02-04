// https://atcoder.jp/contests/arc167/tasks/arc167_a

pub fn run(n: usize, m: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();

    vec.sort();

    let mut b: Vec<usize> = Vec::new();
    let mut c = Vec::new();

    for i in 0..n-m {
        c.push(vec[i]);
    }

    for i in 0..m {
        b.push(vec[i+n-m]);
    }

    c.reverse();

    for i in 0..n-m {
        b[i] += c[i];
    }

    b.iter()
        .map(|num| {
            num * num
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, vec![1, 1, 1, 6, 7], 102),
            TestCase(2, 1, vec![167, 924], 1190281),
            TestCase(12, 9, vec![22847, 98332, 854, 68844, 81080, 46058, 40949, 62493, 76561, 52907, 88628, 99740], 61968950639),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
