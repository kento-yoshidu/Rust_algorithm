// https://atcoder.jp/contests/abc099/tasks/abc099_b

pub fn run(a: usize, b: usize) -> usize {
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

pub fn run2(a: usize, b: usize) -> usize {
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

    #[test]
    fn test() {
        assert_eq!(2, run(8, 13));
        assert_eq!(1, run(54, 65));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(8, 13));
        assert_eq!(1, run2(54, 65));
    }
}
