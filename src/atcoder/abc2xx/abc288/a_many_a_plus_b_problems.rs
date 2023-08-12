// https://atcoder.jp/contests/abc288/tasks/abc288_a

#[allow(dead_code)]
pub fn run(_n: i32, vec: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::<i32>::new();

    for v in vec.iter() {
        ans.push(v[0] + v[1]);
    }

    ans
}

#[allow(dead_code)]
pub fn run2(_n: usize, vec: Vec<(i32, i32)>) -> Vec<i32> {
    vec.iter().map(|v| {
        v.0 + v.1
    }).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![8, -4, -5, 437616054], run(4, vec![vec![3, 5], vec![2, -6], vec![-5, 0], vec![314159265, 123456789]]));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![8, -4, -5, 437616054], run2(4, vec![(3, 5), (2, -6), (-5, 0), (314159265, 123456789)]));
    }
}
