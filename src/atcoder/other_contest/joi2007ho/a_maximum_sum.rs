// https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_a

pub fn run(_n: usize, k: usize, vec: Vec<isize>) -> isize {
    vec.windows(k)
        .map(|t| {
            t.iter().sum::<isize>()
        })
        .max()
        .unwrap()
}

pub fn run2(n: usize, k: usize, vec: Vec<isize>) -> isize {
    let mut total = vec![0; n+1];

    total[0] = 0;

    for (i, num) in vec.iter().enumerate() {
        total[i+1] = total[i] + num;
    }

    (0..(n-k))
        .map(|i| {
            total[k+i] - total[i]
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(5, 3, vec![2, 5, -4, 10, 3]));
    }
}
