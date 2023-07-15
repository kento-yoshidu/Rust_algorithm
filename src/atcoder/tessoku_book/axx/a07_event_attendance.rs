// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_g

#[allow(dead_code)]
pub fn run(d: usize, n: usize, vec: Vec<(i32, i32)>) -> Vec<i32> {
    let mut tmp = vec![0; d];

    for i in 0..n {
        tmp[((vec[i].0) -1) as usize] += 1;
        tmp[((vec[i].1)) as usize] -= 1;
    }

    let mut ans = vec![0; d];


    ans[0] = tmp[0];

    for i in 1..tmp.len() {
        ans[i] = ans[i - 1] + tmp[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1, 2, 2, 2, 2, 1, 1, 2, 2, 1, 1, 1, 1, 0], run(15, 3, vec![(2, 10), (3, 6), (9, 14)]));
        assert_eq!(vec![1, 2, 4, 3, 4, 3, 2, 0], run(8, 5, vec![(2, 3), (3, 6), (5, 7), (3, 7), (1, 5)]));
    }
}
