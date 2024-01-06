// https://atcoder.jp/contests/abc281/tasks/abc281_c

pub fn run(n: isize, t: isize, vec: Vec<isize>) -> (isize, isize) {
    let total: isize = vec.iter().sum();

    // t分の内、残り時間がどれだけあるか
    let mut rest = t % total;

    // 何曲再生されたか
    let mut count = 0;

    loop {
        for i in vec.iter() {
            rest -= i;

            if rest < 0 {
                return (count % n + 1, rest + i)
            }

            // 曲が最後まで再生されたらカウントする
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((1, 60), run(3, 600, vec![180, 240, 120]));
        assert_eq!((3, 93), run(3, 281, vec![94, 94, 94]));
        assert_eq!((6, 678912340), run(10, 5678912340, vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000]))
    }
}
