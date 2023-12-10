// https://atcoder.jp/contests/abc283/tasks/abc283_b

pub fn run(_n: usize, a: Vec<usize>, _q: usize, q_vec: Vec<Vec<usize>>) -> Vec<usize> {
    q_vec.iter()
        .fold((Vec::new(), a), |(mut ans, mut state), vec| {
            match vec[0] {
                1 => {
                    state[vec[1]-1] = vec[2];
                    (ans, state)
                },
                2 => {
                    ans.push(state[vec[1]-1]);
                    (ans, state)
                },
                _ => unreachable!()
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 5, 0, 8, 1], run(3, vec![1, 3, 5], 7, vec![vec![2, 2], vec![2, 3], vec![1, 3, 0], vec![2, 3], vec![1, 2, 8], vec![2, 2], vec![2, 1]]));
        assert_eq!(vec![2, 16, 0, 0, 16, 100], run(5, vec![22, 2, 16, 7, 30], 10, vec![vec![1, 4, 0], vec![1, 5, 0], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![1, 4, 100], vec![1, 5, 100], vec![2, 3], vec![2, 4]]));
        assert_eq!(vec![478, 317, 838, 838, 176, 595, 468], run(7, vec![478, 369, 466, 343, 541, 42, 165], 20, vec![vec![2, 1], vec![1, 7, 729], vec![1, 6, 61], vec![1, 6, 838], vec![1, 3, 319], vec![1, 4, 317], vec![2, 4], vec![1, 1, 673], vec![1, 3, 176], vec![1, 5, 250], vec![1, 1, 468], vec![2, 6], vec![1, 7, 478], vec![1, 5, 595], vec![2, 6], vec![1, 6, 599], vec![1, 6, 505], vec![2, 3], vec![2, 5], vec![2, 1]]));
    }
}
