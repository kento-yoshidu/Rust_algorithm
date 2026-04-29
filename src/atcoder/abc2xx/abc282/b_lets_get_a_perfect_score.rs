// https://atcoder.jp/contests/abc282/tasks/abc282_b

fn run(n: usize, m: usize, s: &Vec<&str>) -> usize {
    let mut ans = 0;

    for i in 0..n-1 {
        'outer: for j in i+1..n {

            for index in 0..m {
                if s[i].chars().nth(index).unwrap() == 'x' && s[j].chars().nth(index).unwrap() == 'x' {
                    break 'outer;
                }
            }

            ans += 1;
        }
    }

    ans
}

fn run2(_u: usize, _m: usize, s: &Vec<&str>) -> usize {
    use itertools::Itertools;

    s.iter()
        .combinations(2)
        .filter(|t| {
            t[0].chars()
                .zip(t[1].chars())
                .all(|(c1, c2)| {
                    c1 == 'o' || c2 == 'o'
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn abc282_b() {
        let tests = [
            TestCase(5, 5, vec!["ooooo", "oooxx", "xxooo", "oxoxo", "xxxxx"], 5),
            TestCase(3, 2, vec!["ox", "xo", "xx"], 1),
            TestCase(2, 4, vec!["xxxx", "oxox"], 0),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, &s), expected);
            assert_eq!(run2(n, m, &s), expected);
        }
    }
}
