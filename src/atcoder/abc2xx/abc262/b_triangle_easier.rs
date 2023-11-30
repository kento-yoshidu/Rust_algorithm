// https://atcoder.jp/contests/abc262/tasks/abc262_b

pub fn run(n: usize, _m: usize, uv: Vec<(usize, usize)>) -> usize {
    let mut connect = vec![vec![false; n]; n];

    for (i, v) in uv.iter() {
        connect[i-1][v-1] = true;
        connect[v-1][i-1] = true;
    }

    let mut ans = 0;

    for a in 0..n {
        for b in a+1..n {
            for c in b+1..n {
                if connect[a][b] == true && connect[b][c] == true && connect[c][a] == true {
                    ans += 1;
                }
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
        assert_eq!(2, run(5, 6, vec![(1, 5), (4, 5), (2, 3), (1, 4), (3, 5), (2, 5)]));
        assert_eq!(0, run(3, 1, vec![(1, 2)]));
        assert_eq!(4, run(7, 10, vec![(1, 7), (5, 7), (2, 5), (3, 6), (4, 7), (1, 5), (2, 4), (1, 3), (1, 6), (2, 7)]));
    }
}
