
// https://atcoder.jp/contests/abc072/tasks/arc082_a

pub fn run(_n: i32, vec: Vec<i32>) -> usize {
    let mut ans = 0;

    let min = vec.iter().min().unwrap();
    let max = vec.iter().max().unwrap();

    for i in *min..=*max {
        let mut count = 0;

        for j in vec.iter() {
            if (*j - i).abs() <= 1 {
                count += 1;
            }
        }

        ans = ans.max(count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(7, vec![3, 1, 4, 1, 5, 9, 2]));
        assert_eq!(3, run(10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(1, run(1, vec![99999]));
    }
}
