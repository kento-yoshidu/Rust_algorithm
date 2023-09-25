// https://atcoder.jp/contests/abc297/tasks/abc297_c

pub fn run(_h: usize, _w: usize, s: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

    #[test]
    fn test() {
        assert_eq!(vec![vec!['P', 'C', 'T'], vec!['T', '.', 'T']], run(2, 3, vec![vec!['T', 'T', 'T'], vec!['T', '.', 'T']]));
        assert_eq!(vec![vec!['P', 'C', 'T', '.', '.'], vec!['.', 'P', 'C', 'T', '.'], vec!['P', 'C', 'P', 'C', 'T']], run(3, 5, vec![vec!['T', 'T', 'T', '.', '.'], vec!['.', 'T', 'T', 'T', '.'], vec!['T', 'T', 'T', 'T', 'T']]));
    }
}
