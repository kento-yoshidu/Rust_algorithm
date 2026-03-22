// https://atcoder.jp/contests/abc269/tasks/abc269_b

fn run(s: Vec<&str>) -> Vec<isize> {
    let vec: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    let mut a = std::isize::MAX;
    let mut b = std::isize::MIN;
    let mut c = std::isize::MAX;
    let mut d = std::isize::MIN;

    for i in 0..10 {
        for j in 0..10 {
            if vec[i][j] == '#' {
                a = a.min(i as isize +1);
                b = b.max(i as isize +1);
                c = c.min(j as isize +1);
                d = d.max(j as isize +1);
            }
        }
    }

    vec![a, b, c, d]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, Vec<isize>);

    #[test]
    fn abc269_b() {
        let tests = [
            TestCase(vec!["..........", "..........", "..........", "..........", "...######.", "...######.", "...######.", "...######.", "..........", ".........." ], vec![5, 8, 4, 9]),
            TestCase(vec!["..........", "..#.......", "..........", "..........", "..........", "..........", "..........", "..........", "..........", ".........."], vec![2, 2, 3, 3]),
            TestCase(vec!["##########", "##########", "##########", "##########", "##########", "##########", "##########", "##########", "##########", "##########"], vec![1, 10, 1, 10]),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
