// https://atcoder.jp/contests/abc115/tasks/abc115_b

pub fn run(_n: usize, p: Vec<usize>) -> usize {
    let mut vec: Vec<usize> = p.clone();

    vec.sort();
    vec.reverse();

    vec.iter()
        .enumerate()
        .map(|(i, price)| {
            if i == 0 {
                price / 2
            } else {
                *price
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(15950, run(3, vec![4980, 7980, 6980]));
        assert_eq!(15120, run(4, vec![4320, 4320, 4320, 4320]));
    }
}
