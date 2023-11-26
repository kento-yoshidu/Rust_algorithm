// https://atcoder.jp/contests/abc182/tasks/abc182_b

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let c = a.clone();

    let max = c.iter().max().unwrap();

    let mut tmp = 0;
    let mut ans = 0;

    for i in 2..=*max {
        let count = a.iter().filter(|&&num| num % i == 0).count();

        if count > tmp {
            tmp = count;
            ans = i;
        }
    }

    ans
}

/*
pub fn run2(_n: usize, a: Vec<usize>) -> usize {
    let max = a.iter().max().unwrap();

    (2..=*max)
        .map(|num| {
            a.iter()
                .filter(|i| {
                    *i % num == 0
                })
                .count()
        })
        .max()
        .unwrap()

    // *vec
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(3, vec![3, 12, 7]));
        assert_eq!(2, run(5, vec![8, 9, 18, 90, 72]));
        assert_eq!(2, run(5, vec![1000, 1000, 1000, 1000, 1000]));
    }

    /*
    #[test]
    fn test2() {
        assert_eq!(3, run2(3, vec![3, 12, 7]));
        assert_eq!(2, run2(5, vec![8, 9, 18, 90, 72]));
        assert_eq!(2, run2(5, vec![1000, 1000, 1000, 1000, 1000]));
    }
    */
}
