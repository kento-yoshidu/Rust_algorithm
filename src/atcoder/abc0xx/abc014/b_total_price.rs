// https://atcoder.jp/contests/abc014/tasks/abc014_2

pub fn run(_n: usize, x: usize, a: Vec<usize>) -> usize {
    if x == 0 {
        return 0;
    }

    let mut b = Vec::new();

    let mut num = x;

    loop {
        if num == 1 {
            b.push(1);
            break
        }

        b.push(num%2);
        num /= 2;
    }

    b.iter()
        .enumerate()
        .map(|(i, b)| {
            if *b == 1 {
                a[i]
            } else {
                0
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(101, run(4, 5, vec![1, 10, 100, 1000]));
        assert_eq!(210, run(20, 1048575, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
        assert_eq!(0, run(4, 0, vec![1000, 1000, 1000]));
    }
}
