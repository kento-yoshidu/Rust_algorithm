/*
// https://atcoder.jp/contests/abc339/tasks/abc339_b

pub fn run(h: usize, w: usize, n: usize) -> Vec<String> {
    let mut vec = vec![vec!['.'; w]; h];

    let di: [isize; 4] = [-1, 0, 1, 0];
    let dj: [isize; 4] = [0, 1, 0, -1];

    let mut dir = 0;
    let mut i = 0;
    let mut j = 0;

    for _ in 0..n {
        if vec[i as usize][j as usize] == '.' {
            vec[i as usize][j as usize] = '#';
            dir += 1;
        } else {
            vec[i as usize][j as usize] = '.';
            dir += 3;
        }

        dir %= 4;

        i += di[dir];
        j += dj[dir];

        i = (i as usize + h) % h;
        j = (j as usize + w) % w;
    }

    vec.iter()
        .map(|v| {
            v.iter().collect::<String>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>);

    #[test]
    fn ts() {
        let tests = [
            TestCase(3, 4, 5, vec![".#..", "##..", "...."]),
            TestCase(2, 2, 1000, vec!["..", ".."]),
            TestCase(10, 10, 10, vec!["##........",  "##........", "..........", "..........", "..........", "..........", "..........", "..........", "..........", "#........#"]),
        ];

        for TestCase(h, w, n, expected) in tests {
            assert_eq!(run(h, w, n), expected);
        }
    }
}
*/
