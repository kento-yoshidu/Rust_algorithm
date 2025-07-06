// https://atcoder.jp/contests/abc099/tasks/abc099_b

fn run(a: usize, b: usize) -> usize {
    let mut cum_sum = Vec::new();

    for i in 0..=(b-a) {
        if i == 0 {
            cum_sum.push(i)
        } else {
            cum_sum.push(cum_sum[i-1] + i);
        }
    }

    *cum_sum.iter().last().unwrap() - b
}

fn run2(a: usize, b: usize) -> usize {
    let cum_sum: Vec<usize> = (0..=(b-a))
        .scan(Vec::new(), |cum_sum, i| {
            if cum_sum.len() == 0 {
                cum_sum.push(i);
            } else {
                cum_sum.push(cum_sum[i-1] + i);
            }

            Some(*cum_sum.iter().last().unwrap())
        })
        .collect();

    *cum_sum.iter().last().unwrap() - b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 13, 2),
            TestCase(54, 65, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
            assert_eq!(run2(a, b), expected);
        }
    }
}
