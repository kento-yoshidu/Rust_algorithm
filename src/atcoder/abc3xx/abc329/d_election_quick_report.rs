// https://atcoder.jp/contests/abc329/tasks/abc329_d

pub fn run(n: usize, _m: usize, a: Vec<usize>) -> Vec<usize> {
    let mut arr = vec![0; n];
    let mut current = (0, 0);

    let mut ans: Vec<usize> = Vec::new();

    for i in a.iter() {
        arr[i-1] += 1;

        if arr[i-1] > current.1 {
            current = (*i, current.1 + 1);
        } else if arr[i-1] == current.1 {
            current = (*i.min(&current.0), current.1);
        }

        ans.push(current.0)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 1, 2, 2, 1, 1, 3], run(3, 7, vec![1, 2, 2, 3, 1, 3, 3]));
        assert_eq!(vec![100, 90, 80, 70, 60], run(100, 5, vec![100, 90, 80, 70, 60]));
        assert_eq!(vec![8, 8, 8, 2, 8, 8, 8, 2], run(9, 8, vec![8, 8, 2, 2, 8, 8, 2, 2]));
    }
}
