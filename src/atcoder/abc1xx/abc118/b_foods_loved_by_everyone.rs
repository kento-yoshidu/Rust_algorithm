// https://atcoder.jp/contests/abc118/tasks/abc118_b

pub fn run(_n: usize, m: usize, v: Vec<Vec<usize>>) -> usize {
    let vec: Vec<Vec<usize>> = v.iter()
        .map(|vec| {
            vec.iter()
                .enumerate()
                .filter(|(i, _)| *i != 0)
                .map(|(_, v)| *v)
                .collect()
        })
        .collect();

    (1..=m)
        .filter(|num| {
            vec.iter()
                .all(|vec| {
                    vec.contains(num)
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 4, vec![vec![2, 1, 3], vec![3, 1, 2, 3], vec![2, 3, 2]]));
        assert_eq!(0, run(5, 5, vec![vec![4, 2, 3, 4, 5], vec![4, 1, 3, 4, 5], vec![4, 1, 2, 4, 5], vec![4, 1, 2, 3, 5], vec![4, 1, 2, 3, 4,]]));
        assert_eq!(3, run(1, 30, vec![vec![3, 5, 10, 30]]));
    }
}
