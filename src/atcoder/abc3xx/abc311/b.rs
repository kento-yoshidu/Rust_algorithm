// https://atcoder.jp/contests/abc311/tasks/abc311_b

fn run(_n: usize, d: usize, s: &Vec<&str>) -> usize {
    let s: Vec<Vec<char>> = s.iter().map(|v| v.chars().collect()).collect();

    let mut ans = 0;
    let mut streak = 0;

    for day in 0..d {
        if s.iter().all(|v| v[day] == 'o') {
            streak += 1;
            ans = ans.max(streak);
        } else {
            streak = 0;
        }
    }

    ans
}

fn run2(_n: usize, d: usize, s: &Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.iter().map(|v| v.chars().collect()).collect();

    (0..d)
        .fold((0, 0), |(ans, state), day| {
            if vec.iter().all(|v| v[day] == 'o') {
                (ans.max(state + 1), state + 1)
            } else {
                (ans, 0)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn abc311_b() {
        let tests = [
            TestCase(3, 5, vec!["xooox", "oooxx", "oooxo"], 2),
            TestCase(3, 3, vec!["oxo", "oxo", "oxo"], 1),
            TestCase(3, 3, vec!["oox", "oxo", "xoo"], 0),
            TestCase(1, 7, vec!["ooooooo"], 7),
            TestCase(5, 15, vec!["oxooooooooooooo", "oxooxooooooooox", "oxoooooooooooox", "oxxxooooooxooox", "oxooooooooxooox"], 5),
        ];

        for TestCase(n, d, s, expected) in tests {
            assert_eq!(run(n, d, &s), expected);
            assert_eq!(run2(n, d, &s), expected);
        }
    }
}
