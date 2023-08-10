// https://atcoder.jp/contests/arc025/tasks/arc025_1

fn run(dd: Vec<usize>, jj: Vec<usize>) -> usize {
    let mut ans = 0;

    for bit in 0..(1 << dd.len()) {
        let mut total = 0;

        for i in 0..dd.len() {
            if bit & (1 << i) != 0 {
                total += dd[i];
            } else {
                total += jj[i];
            }
        }

        ans = ans.max(total);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(33, run(vec![4, 2, 0, 5, 6, 2, 5], vec![6, 1, 4, 3, 6, 4, 6]));
        assert_eq!(35, run(vec![1, 2, 3, 4, 5, 6, 7], vec![2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(0, run(vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(793, run(vec![8, 3, 0, 2, 5, 25, 252], vec![252, 252, 2, 5, 2, 5, 2]));
    }
}
