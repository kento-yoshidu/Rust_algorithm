// https://atcoder.jp/contests/abc269/tasks/abc269_b

pub fn run(s: Vec<&str>) -> Vec<i32> {
    let vec: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    let mut a = std::i32::MAX;
    let mut b = std::i32::MIN;
    let mut c = std::i32::MAX;
    let mut d = std::i32::MIN;

    for i in 0..10 {
        for j in 0..10 {
            if vec[i][j] == '#' {
                a = a.min(i as i32 +1);
                b = b.max(i as i32 +1);
                c = c.min(j as i32 +1);
                d = d.max(j as i32 +1);
            }
        }
    }

    vec![a, b, c, d]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![5, 8, 4, 9], run(
        vec!["..........",
                "..........",
                "..........",
                "..........",
                "...######.",
                "...######.",
                "...######.",
                "...######.",
                "..........",
                ".........." ]));
    assert_eq!(vec![2, 2, 3, 3], run(
        vec!["..........",
                "..#.......",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                ".........."]));
    assert_eq!(vec![1, 10, 1, 10], run(
        vec!["##########",
                "##########",
                "##########",
                "##########",
                "##########",
                "##########",
                "##########",
                "##########",
                "##########",
                "##########"]));
    }
}
