// https://atcoder.jp/contests/abc303/tasks/abc303_b

pub fn run(n: usize, _m: usize, a: Vec<Vec<usize>>) -> usize {
    let mut map = vec![vec![false; n]; n];

    for i in 0..n {
        map[i][i] = true;
    }

    for vec in a.iter() {
        for i in 0..n-1 {
            map[vec[i]-1][vec[i+1]-1] = true;
            map[vec[i+1]-1][vec[i]-1] = true;
        }
    }

    map.iter()
        .map(|vec| {
            vec.iter()
                .filter(|ele| {
                    **ele == false
                })
                .count()
        })
        .sum::<usize>() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, 2, vec![vec![1, 2, 3, 4], vec![4, 3, 1, 2]]));
        assert_eq!(0, run(3, 3, vec![vec![1, 2, 3], vec![3, 1, 2], vec![1, 2 ,3]]));
        assert_eq!(6, run(10, 10, vec![
                                    vec![4, 10, 7, 2, 8, 3, 9, 1, 6, 5],
                                    vec![3, 6, 2, 9, 1, 8, 10, 7, 4, 5],
                                    vec![9, 3, 4, 5, 7, 10, 1, 8, 2, 6],
                                    vec![7, 3, 1, 8, 4, 9, 5, 6, 2, 10],
                                    vec![5, 2, 1, 4, 10, 7, 9, 8, 3, 6],
                                    vec![5, 8, 1, 6, 9, 3, 2, 4, 7, 10],
                                    vec![8, 10, 3, 4, 5, 7, 2, 9, 6, 1],
                                    vec![3, 10, 2, 7, 8, 5, 1, 4, 9, 6],
                                    vec![10, 6, 1, 5, 4, 2, 3, 8, 9, 7],
                                    vec![4, 5, 9, 1, 8, 2, 7, 6, 3, 10]]));
    }
}
