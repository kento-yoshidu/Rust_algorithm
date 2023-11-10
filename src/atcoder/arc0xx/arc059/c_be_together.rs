// https://atcoder.jp/contests/abc043/tasks/arc059_a

pub fn run(_n: isize, a: Vec<isize>) -> isize {
    let min = *a.iter().min().unwrap();
    let max = *a.iter().max().unwrap();

    let mut ans = std::isize::MAX;

    for num in min..=max {
        let mut tmp = 0;

        for i in a.iter() {
            tmp += (num - i).pow(2);
        }

        ans = ans.min(tmp);
    }

    ans
}

pub fn run2(_n: isize, a: Vec<isize>) -> isize {
    let min = *a.iter().min().unwrap();
    let max = *a.iter().max().unwrap();

    (min..=max)
        .map(|num| {
            a.iter()
                .map(|i| {
                    (num - i).pow(2)
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(2, vec![4, 8]));
        assert_eq!(3, run(3, vec![1, 1, 3]));
        assert_eq!(5, run(3, vec![4, 2, 5]));
        assert_eq!(0, run(4, vec![-100, -100, -100, -100]));
    }

    #[test]
    fn test2() {
        assert_eq!(8, run2(2, vec![4, 8]));
        assert_eq!(3, run2(3, vec![1, 1, 3]));
        assert_eq!(5, run2(3, vec![4, 2, 5]));
        assert_eq!(0, run2(4, vec![-100, -100, -100, -100]));
    }
}
