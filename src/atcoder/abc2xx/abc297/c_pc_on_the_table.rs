// https://atcoder.jp/contests/abc297/tasks/abc297_c

fn run(_h: usize, _w: usize, s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ans = s.clone();

    for v in ans.iter_mut() {
        for i in 0..v.len()-1 {
            if v[i] == 'T' && v[i+1] == 'T' {
                v[i] = 'P';
                v[i+1] = 'C';
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<char>>, Vec<Vec<char>>);

    #[test]
    fn abc297_c() {
        let tests = [
            TestCase(2, 3, vec![vec!['T', 'T', 'T'], vec!['T', '.', 'T']], vec![vec!['P', 'C', 'T'], vec!['T', '.', 'T']]),
            TestCase(3, 5, vec![vec!['T', 'T', 'T', '.', '.'], vec!['.', 'T', 'T', 'T', '.'], vec!['T', 'T', 'T', 'T', 'T']], vec![vec!['P', 'C', 'T', '.', '.'], vec!['.', 'P', 'C', 'T', '.'], vec!['P', 'C', 'P', 'C', 'T']]),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
