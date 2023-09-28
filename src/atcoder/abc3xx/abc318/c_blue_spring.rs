// https://atcoder.jp/contests/abc318/tasks/abc318_c

pub fn run(n: usize, d: usize, p: usize, f: Vec<usize>) -> usize {
    let mut vec = f.clone();
    vec.sort();
    vec.reverse();

    // フリーパスを使わずに通常料金で乗った場合
    // これを基準にフリーパスを適用した場合の料金と比較する
    let mut ans: usize = f.iter().sum();

    // フリーパスを1枚から使えるだけ使った場合まで試行する
    for i in 1..=((n as f64 / d as f64)).ceil() as usize {
        // フリーパスの料金
        let mut tmp = p * i;

        // フリーパスを使う日がn日未満なら、通常料金を加算する
        if i*d < n {
            // 降順に並んでいるので、先頭からi*d日分を飛ばして加算する
            tmp += &vec[i*d..].iter().sum();
        }
        // フリーパスを使う日がn日以上なら、フリーパスの料金のみなので何もしない

        ans = ans.min(tmp);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(20, run(5, 2, 10, vec![7, 1, 6, 3, 6]));
        assert_eq!(6, run(3, 1, 10, vec![1, 2, 3]));
        assert_eq!(3000000000, run(8, 3, 1000000000, vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000]));
    }
}
