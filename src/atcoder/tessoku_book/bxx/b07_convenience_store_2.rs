// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_al

#[allow(dead_code)]
pub fn run(t: usize, n: usize, vec: Vec<(i32, i32)>) -> Vec<i32> {
    let mut tmp = vec![0; t];

    for i in 0..n {
        tmp[((vec[i].0)) as usize] += 1;
        tmp[((vec[i].1)) as usize] -= 1;
    }

    let mut ans = vec![0; t];

    ans[0] = tmp[0];

    for i in 1..t {
        ans[i] = ans[i - 1] + tmp[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 3, 4, 1, 0, 3, 0, 0, 0, 0], run(10, 7, vec![(0, 3), (2, 4), (1, 3), (0, 3), (5, 6), (5, 6), (5, 6)]));
    }
}