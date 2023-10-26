// https://atcoder.jp/contests/abc325/tasks/abc325_b

pub fn run(_n: usize, vec: Vec<(usize, usize)>) -> usize {
    let mut ans = 0;

    for i in 0..24 {
        let mut count = 0;

        for (w, x) in vec.iter() {
            // 現地時間
            let time = (i+x)%24;

            if 9 <= time && time <= 17 {
                count += w;
            }

            // 17時を過ぎれば無駄なので飛ばす
            if 17 < time {
                break
            }
        }

        ans = ans.max(count)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(3, vec![(5, 0), (3, 3), (2, 18)]));
        assert_eq!(1000000, run(2, vec![(1, 10), (1000000, 20)]));
        assert_eq!(67, run(6, vec![(31, 3), (20, 8), (11, 5), (4, 3), (47, 14), (1, 18)]));
    }
}
