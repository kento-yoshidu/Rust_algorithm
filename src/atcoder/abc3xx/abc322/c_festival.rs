// https://atcoder.jp/contests/abc322/tasks/abc322_c

pub fn run(n: usize, m: usize, a: Vec<usize>) -> Vec<usize> {
    let mut ans: Vec<usize> = Vec::new();
    let mut start = 0;

    for i in 0..n {
        for day in start..m {
            if i+1 <= a[day] {
                ans.push(a[day] - i-1);
                start = day;
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 0, 0], run(3, 2, vec![2, 3]));
        assert_eq!(vec![0, 1, 0, 0, 2, 1, 0, 0], run(8, 5, vec![1, 3, 4, 7, 8]));
    }
}
