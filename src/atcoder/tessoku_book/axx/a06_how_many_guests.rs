// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_ai

#[allow(dead_code, unused)]
pub fn run(n: usize, q: usize, a: Vec<usize>, l: Vec<Vec<usize>>) -> Vec<usize> {
    let mut total: Vec<usize> = Vec::from([0]);

    // 累計和を求める
    for i in 0..n {
        total.push(total[i] + a[i]);
    }

    let mut ans: Vec<usize> = Vec::new();

    for v in l.iter() {
        ans.push(total[v[1]] - total[v[0]-1]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![15, 24, 1123, 111, 11137], run(10, 5, vec![8, 6, 9, 1, 2, 1, 10, 100, 1000, 10000], vec![vec![2, 3], vec![1, 4], vec![3, 9], vec![6, 8], vec![1, 10]]));
    }
}
