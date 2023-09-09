// https://atcoder.jp/contests/abc318/tasks/abc318_b

pub fn run(_n: usize, a: Vec<Vec<usize>>) -> usize {
    let mut filed = vec![vec![false; 101]; 101];

    for v in a {
        for x in v[0]..v[1] {
            for y in v[2]..v[3] {
                filed[x][y] = true
            }
        }
    }

    filed.iter().map(|vec| {
        vec.iter().filter(|v| {
            **v == true
        }).count()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(20, run(3, vec![vec![0, 5, 1, 3], vec![1, 4, 0, 5], vec![2, 5, 2, 4]]));
        assert_eq!(10000, run(2, vec![vec![0, 100, 0, 100], vec![0, 100, 0, 100]]));
        assert_eq!(65, run(3, vec![vec![0, 1, 0, 1], vec![0, 3, 0, 5], vec![5, 10, 0, 10]]));
    }
}
