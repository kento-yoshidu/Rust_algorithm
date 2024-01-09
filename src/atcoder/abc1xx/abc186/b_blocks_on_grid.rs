// https://atcoder.jp/contests/abc186/tasks/abc186_b

pub fn run(_h: usize, _w: usize, a: Vec<Vec<usize>>) -> usize {
    let min = a.iter()
        .map(|v| {
            v.iter().min().unwrap()
        })
        .min()
        .unwrap();

    a.iter()
        .map(|v| {
            v.iter().map(|num| {
                num - min
            })
            .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(2, 3, vec![vec![2, 2, 3], vec![3, 2, 2]]));
        assert_eq!(792, run(3, 3, vec![vec![99, 99, 99], vec![99, 0, 99], vec![99, 99, 99]]));
        assert_eq!(0, run(3, 2, vec![vec![4, 4], vec![4, 4], vec![4, 4]]));
    }
}
