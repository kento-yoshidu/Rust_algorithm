// https://atcoder.jp/contests/abc240/tasks/abc240_d

pub fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    a.iter()
        // state = (番号、個数)というタプルで状態を管理
        .scan((0, vec![(0, 0)]), |(ans, state), num| {
            // ボールを入れた時点でとりあえず+1
            *ans += 1;

            let len = state.len();

            // 末尾と同じ番号なら個数を+1
            if state[len-1].0 == *num {
                state[len-1].1 += 1;
            } else {
                // 違う番号ならpush
                state.push((*num, 1));
            }

            if state[len-1].0 == *num && state[len-1].1 == *num {
                state.pop();
                *ans -= *num;
            }

            Some(*ans)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 2, 3, 4, 3], run(5, vec![3, 2, 3, 2, 2]));
        assert_eq!(vec![1, 2, 3, 4, 5, 3, 2, 3, 1, 0], run(10, vec![2, 3, 2, 3, 3, 3, 2, 3, 3, 2]));
        assert_eq!(vec![1, 2, 3, 4, 5, 3, 2, 3, 1, 0], run(10, vec![2, 3, 2, 3, 3, 3, 2, 3, 3, 2]));
    }
}
