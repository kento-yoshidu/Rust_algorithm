// https://atcoder.jp/contests/abc317/tasks/abc317_b

pub fn run(_n: usize, a: &mut Vec<usize>) -> usize {
    a.sort();

    a.windows(2).find(|arr| {
        arr[1] - arr[0] > 1
    }).unwrap()[0] + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(3, &mut vec![2, 3, 5]));
        assert_eq!(7, run(8, &mut vec![3, 1, 4, 5, 9, 2, 6, 8]));
        assert_eq!(151, run(16, &mut vec![152, 153, 154, 147, 148, 149, 158, 159, 160, 155, 156, 157, 144, 145, 146, 150]));
    }
}
