// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/7/ITP1_7_B

fn calc(n: usize, x: usize) -> usize {
    let mut sum = 0;

    for i in 1..=n {
        for j in i+1..=n {
            let k = x as i32 - i as i32 - j as i32;
            if k > j as i32 && k <= n as i32 {
                sum += 1;
            }
        }
    }

    sum
}

fn run(nx: Vec<(usize, usize)>) -> Vec<usize> {
    nx.into_iter()
        .filter_map(|(n, x)| {
            if n == 0 && x == 0 {
                None
            } else {
                Some(calc(n, x))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn itp1_7_b() {
        let tests = [
            TestCase(vec![(5, 9), (0, 0)], vec![2]),
        ];

        for TestCase(nx, expected) in tests {
            assert_eq!(run(nx), expected);
        }
    }
}
