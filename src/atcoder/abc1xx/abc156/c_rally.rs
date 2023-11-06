// https://atcoder.jp/contests/abc156/tasks/abc156_c

pub fn run(_n: isize, vec: Vec<isize>) -> usize {
    let mut ans = 10000;

    for i in 0..=100 {
        let mut sum = 0;

        for v in vec.iter() {
            sum += (v - (i+1) as isize).pow(2)
        }

        ans = ans.min(sum);
    }

    ans as usize
}

pub fn run2(_n: usize, x: Vec<isize>) -> isize {
    (0..=100)
        .map(|i| {
            x.iter()
                .fold(0, |state, x| {
                    state + (x - i+1).pow(2)
                })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(2, vec![1, 4]));
        assert_eq!(2354, run(7, vec![14, 14, 2, 13, 56, 2, 37]));
    }

    #[test]
    fn test2() {
        assert_eq!(5, run2(2, vec![1, 4]));
        assert_eq!(2354, run2(7, vec![14, 14, 2, 13, 56, 2, 37]));
    }
}
