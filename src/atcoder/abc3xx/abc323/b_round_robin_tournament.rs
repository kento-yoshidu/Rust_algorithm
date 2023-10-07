// https://atcoder.jp/contests/abc323/tasks/abc323_b

pub fn run(_n: usize, s: Vec<&str>) -> Vec<usize> {
    let mut vec: Vec<(usize, usize)> = Vec::new();

    for (index, v) in s.iter().enumerate() {
        let count = v.chars().filter(|c| *c == 'o').count();

        vec.push((index+1, count));
    }

    vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    vec.iter().map(|t| {
        t.0
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 2, 1], run(3, vec!["-xx", "o-x", "oo-"]));
        assert_eq!(vec![4, 7, 3, 1, 5, 2, 6], run(7, vec!["-oxoxox", "x-xxxox", "oo-xoox", "xoo-ooo", "ooxx-ox", "xxxxx-x", "oooxoo-"]));
    }
}
