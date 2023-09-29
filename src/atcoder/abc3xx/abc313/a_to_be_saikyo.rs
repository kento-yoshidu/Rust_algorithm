// https://atcoder.jp/contests/abc313/tasks/abc313_a

fn run(n: usize, vec: Vec<usize>) -> usize {
    if n == 1 {
        return 0
    }

    let max = vec.iter().skip(1).max().unwrap();

    if vec[0] > *max {
        0
    } else {
        max - vec[0] + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(4, vec![5, 15, 2, 10]));
        assert_eq!(0, run(4, vec![15, 5, 2, 10]));
        assert_eq!(1, run(3, vec![100, 100, 100]));
        assert_eq!(0, run(1, vec![1]));
    }
}
