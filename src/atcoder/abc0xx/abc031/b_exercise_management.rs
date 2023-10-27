// https://atcoder.jp/contests/abc031/tasks/abc031_b

pub fn run(l: isize, h: isize, _n: usize, v: Vec<isize>) -> Vec<isize> {
    v.iter()
        .map(|&i| {
            if i < l { l - i}
            else if h < i { -1 }
            else { 0 }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![60, 0, -1], run(300, 400, 3, vec![240, 350, 480]));
        assert_eq!(vec![-1, 0, -1, 0, 50], run(50, 80, 5, vec![10000, 50, 81, 80, 0]));
    }
}
