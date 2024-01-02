// https://atcoder.jp/contests/agc039/tasks/agc039_a

use itertools::Itertools;

pub fn run(s: &str, k: usize) -> usize {
    // 全て同じ文字だった場合
    if s.chars().all_equal() {
        return s.len() * k / 2
    }

    let mut i = 0;
    let mut run_lengths = vec![];
    let mut current = (s.chars().nth(0).unwrap(), 0);

    // ランレングス圧縮
    loop {
        while i < s.len() && s.chars().nth(i).unwrap() == current.0 {
            current.1 += 1;
            i += 1;
        }

        run_lengths.push(current);

        if i == s.len() {
            break;
        } else {
            current = (s.chars().nth(i).unwrap(), 0);
        }
    }

    // 最初と最後の文字が違うなら、連続している数 / 2を合計してk倍して返す
    if s.chars().nth(0) != s.chars().last() {
        run_lengths.iter()
            .map(|(_, num)| num / 2 )
            .sum::<usize>() * k
    } else {
        let mut ans = 0;

        // 両端以外を合計する
        for i in 1..run_lengths.len()-1 {
            ans += run_lengths[i].1 / 2;
        }

        let left = run_lengths[0].1;
        let right = run_lengths.iter().last().unwrap().1;

        ans * k + (left + right) / 2 * (k - 1) + left/2 + right/2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run("issii", 2));
        assert_eq!(81, run("qq", 81));
        assert_eq!(8999939997, run("cooooooooonteeeeeeeeeest", 999993333));
        assert_eq!(50000000000, run("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 1000000000));
        assert_eq!(49499999950, run("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 999999999));
    }
}
