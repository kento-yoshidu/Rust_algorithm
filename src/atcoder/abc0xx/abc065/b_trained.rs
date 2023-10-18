// https://atcoder.jp/contests/abc065/tasks/abc065_b

pub fn run(n: usize, a: Vec<usize>) -> isize {
    let mut count = 0;
    let mut point = 0;

    loop {
        if a[point] == 2 {
            return (count+1) as isize
        }

        count += 1;
        point = a[point]-1;

        if count == n {
            return -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, vec![3, 1, 2]));
        assert_eq!(-1, run(4, vec![3, 4, 1, 2]));
        assert_eq!(3, run(5, vec![3, 3, 4, 2, 4]));
    }
}
