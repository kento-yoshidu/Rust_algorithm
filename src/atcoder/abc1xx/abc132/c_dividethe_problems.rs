// https://atcoder.jp/contests/abc132/tasks/abc132_c

pub fn run(k: usize, vec: &mut Vec<i32>) -> i32 {
    vec.sort();

    let abc = vec[k/2-1];
    let arc = vec[k/2];

    arc - abc
}

// Refactoring
// 計算量削減
pub fn run2(_n: usize, d: Vec<usize>) -> usize {
    let min = d.iter().min().unwrap();
    let max = d.iter().max().unwrap();

    (*min..=*max)
        .map(|border| {
            d.iter().partition(|i| {
                border < **i
            })
        })
        .filter(|t: &(Vec<usize>, Vec<usize>)| {
            t.0.len() == t.1.len()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, &mut vec![9, 1, 4, 4, 6, 7]));
        assert_eq!(0, run(8, &mut vec![9, 1, 14, 5, 5, 4, 4, 14]));
        assert_eq!(42685, run(14, &mut vec![99592, 10342, 29105, 78532, 83018, 11639, 92015, 77204, 30914, 21912, 34519, 80835, 100000, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(6, vec![9, 1, 4, 4, 6, 7]));
        assert_eq!(0, run2(8, vec![9, 1, 14, 5, 5, 4, 4, 14]));
        assert_eq!(42685, run2(14, vec![99592, 10342, 29105, 78532, 83018, 11639, 92015, 77204, 30914, 21912, 34519, 80835, 100000, 1]));
    }
}
