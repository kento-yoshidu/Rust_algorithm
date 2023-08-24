// https://atcoder.jp/contests/abc311/tasks/abc311_b

#[allow(dead_code)]
fn run(_n: usize, d: usize, vec: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = vec.iter().map(|v| {
        v.chars().collect::<Vec<char>>()
    }).collect();

    let mut ans = 0;
    let mut streak = 0;

    for day in 0..d {
        if vec.iter().all(|v| v[day] == 'o') {
            streak += 1;
            ans = ans.max(streak);
        } else {
            streak = 0;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, 5, vec!["xooox", "oooxx", "oooxo"]));
        assert_eq!(1, run(3, 3, vec!["oxo", "oxo", "oxo"]));
        assert_eq!(0, run(3, 3, vec!["oox", "oxo", "xoo"]));
        assert_eq!(7, run(1, 7, vec!["ooooooo"]));
        assert_eq!(5, run(5, 15, vec!["oxooooooooooooo", "oxooxooooooooox", "oxoooooooooooox", "oxxxooooooxooox", "oxooooooooxooox"]));
    }
}
