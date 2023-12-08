// https://atcoder.jp/contests/abc134/tasks/abc134_c

pub fn run(_n: usize, p: Vec<usize>) -> Vec<usize> {
    let mut vec = p.clone();
    vec.sort();
    vec.reverse();

    let max = vec[0];
    let next = vec[1];

    p.iter()
        .map(|num| {
            if *num == max {
                next
            } else {
                max
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![4, 3, 4], run(3, vec![1, 4, 3]));
        assert_eq!(vec![5, 5, 5, 5, 1], run(3, vec![1, 1, 1, 1, 5]));
        assert_eq!(vec![5, 5], run(2, vec![5, 5]));
    }
}
