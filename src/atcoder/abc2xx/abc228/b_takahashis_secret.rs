// https://atcoder.jp/contests/abc228/tasks/abc228_b

fn check(a: &Vec<usize>, mut vec: Vec<bool>, count: usize, current: usize) -> usize {
    if vec[current-1] == true {
        count
    } else {
        vec[current-1] = true;
        check(a, vec, count+1, a[current-1])
    }
}

pub fn run(n: usize, x: usize, a: Vec<usize>) -> usize {
    let vec = vec![false; n];

    check(&a, vec, 0, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, 2, vec![3, 1, 1, 2]));
        assert_eq!(7, run(20, 12, vec![7, 11, 10, 1, 7, 20, 14, 2, 17, 3, 2, 5, 19, 20, 8, 14, 18, 2, 10, 10]));
    }
}
