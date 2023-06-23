// https://atcoder.jp/contests/abc243/tasks/abc243_b

#[allow(dead_code)]
pub fn run(_n: i32, a: Vec<i32>, b: Vec<i32>) -> (i32, i32) {
    let mut count_a = 0;
    let mut count_b = 0;

    for (i, aa) in a.iter().enumerate() {
        for (j, bb) in b.iter().enumerate() {
            if i == j && aa == bb {
                count_a += 1;
            } else if aa == bb {
                count_b += 1;
            }
        }
    }

    (count_a, count_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((1, 2), run(4, vec![1, 3, 5, 2], vec![2, 3, 1, 4]));
        assert_eq!((0, 0), run(3, vec![1, 2, 3], vec![4, 5, 6]));
        assert_eq!((3, 2), run(7, vec![4, 8, 1, 7, 9, 5, 6], vec![3, 5, 1, 7, 8, 2, 6]));
    }
}
