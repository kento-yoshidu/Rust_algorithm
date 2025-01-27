// https://atcoder.jp/contests/abc081/tasks/abc081_b

fn run(_n: usize, s: &Vec<usize>) -> usize {
    let result: Vec<usize> = s.iter().map(|&i| {
        let mut count = 0;
        let mut tmp = i;

        loop {
            if tmp % 2 == 0 {
                tmp = tmp >> 1;
                count += 1;
            } else {
                break;
            }
        }

        count
    }).collect();

    result.into_iter().min().unwrap()
}

fn rec(num: usize, count: usize) -> usize {
    if num % 2 != 0 {
        count
    } else {
        rec(num/2, count+1)
    }
}

fn run2(_n: usize, a: &Vec<usize>) -> usize {
    a.into_iter()
        .map(|num| {
            rec(*num, 0)
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![8, 12, 40], 2),
            TestCase(4, vec![5, 6, 8, 10], 0),
            TestCase(6, vec![382253568, 723152896, 37802240, 379425024, 404894720, 471526144], 8),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
