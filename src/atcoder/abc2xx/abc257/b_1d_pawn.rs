
// https://atcoder.jp/contests/abc257/tasks/abc257_b

fn run(n: usize, _k: usize, _q: usize, a: Vec<usize>, l: Vec<usize>) -> Vec<usize> {
    l.into_iter()
        .fold(a.clone(), |mut state, num| {
            if state.contains(&(state[num-1] + 1)) {
                state
            } else if state[num-1] + 1 >= n {
                state[num-1] = n;
                state
            } else {
                state[num-1] += 1;
                state
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 4, 5], run(5, 3, 5, vec![1, 3, 4], vec![3, 3, 1, 1, 2]));
        assert_eq!(vec![1, 2], run(2, 2, 2, vec![1, 2], vec![1, 2]));
        assert_eq!(vec![2, 5, 6, 7, 9, 10], run(10, 6, 9, vec![1, 3, 5, 7, 8, 9], vec![1, 2, 3, 4, 5, 6, 5, 6, 2]));
    }
}
