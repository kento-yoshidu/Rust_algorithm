// https://atcoder.jp/contests/abc281/tasks/abc281_c

fn run(n: isize, t: isize, a: Vec<isize>) -> (isize, isize) {
    let total: isize = a.iter().sum();

    // t分の内、残り時間がどれだけあるか
    let mut rest = t % total;

    // 何曲再生されたか
    let mut count = 0;

    loop {
        for i in a.iter() {
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

    struct TestCase(isize, isize, Vec<isize>, (isize, isize));

    #[test]
    fn abc281_c() {
        let tests = [
            TestCase(3, 600, vec![180, 240, 120], (1, 60)),
            TestCase(3, 281, vec![94, 94, 94], (3, 93)),
            TestCase(10, 5678912340, vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000], (6, 678912340)),
        ];

        for TestCase(n, t, a, expected) in tests {
            assert_eq!(run(n, t, a), expected);
        }
    }
}
