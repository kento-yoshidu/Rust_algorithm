// https://atcoder.jp/contests/abc286/tasks/abc286_a

fn run(_n: usize, p: usize, q: usize, r: usize, s: usize, a: &Vec<usize>) -> Vec<usize> {
    let mut vec = a.clone();

    (p..=q).zip(r..=s).for_each(|v| {
        vec.swap(v.0-1, v.1-1);
    });

    vec
}

fn run2(_n: usize, p: usize, q: usize, r: usize, s: usize, a: &Vec<usize>) -> Vec<usize> {
    a.iter()
        .enumerate()
        .map(|(i, x)| {
            if (p..=q).contains(&(i + 1)) {
                a[i - p + r].clone()
            } else if (r..=s).contains(&(i + 1)) {
                a[i - r + p].clone()
            } else {
                *x
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a>(usize, usize, usize, usize, usize, &'a Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let vec2 = vec![2, 2, 1, 1, 1];
        let vec3 = vec![50, 100];
        let vec4 = vec![22, 75, 26, 45, 72, 81, 47, 29, 97, 2];

        let tests = [
            TestCase(8, 1, 3, 5, 7, &vec1, vec![5, 6, 7, 4, 1, 2, 3, 8]),
            TestCase(5, 2, 3, 4, 5, &vec2, vec![2, 1, 1, 2, 1]),
            TestCase(2, 1, 1, 2, 2, &vec3, vec![100, 50]),
            TestCase(10, 2, 4, 7, 9, &vec4, vec![22, 47, 29, 97, 72, 81, 75, 26, 45, 2]),
        ];

        for TestCase(n, p, q, r, s, a, expected) in tests {
            assert_eq!(run(n, p, q, r, s, a), expected);
            assert_eq!(run2(n, p, q, r, s, a), expected);
        }
    }
}
