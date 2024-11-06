// https://atcoder.jp/contests/joi2009yo/tasks/joi2009yo_a

fn run(
    ah1: usize, am1: usize, as1: usize, ah2: usize, am2: usize, as2: usize,
    bh1: usize, bm1: usize, bs1: usize, bh2: usize, bm2: usize, bs2: usize,
    ch1: usize, cm1: usize, cs1: usize, ch2: usize, cm2: usize, cs2: usize
) -> [String; 3] {
    let a1 = ah1*3600 + am1*60 + as1;
    let a2 = ah2*3600 + am2*60 + as2;
    let a = a2 - a1;

    let b1 = bh1*3600 + bm1*60 + bs1;
    let b2 = bh2*3600 + bm2*60 + bs2;
    let b = b2 - b1;

    let c1 = ch1*3600 + cm1* 60 + cs1;
    let c2 = ch2*3600 + cm2* 60 + cs2;
    let c = c2 - c1;


    [
        format!("{} {} {}", a / 3600, (a / 60) % 60, a % 60),
        format!("{} {} {}", b / 3600, (b / 60) % 60, b % 60),
        format!("{} {} {}", c / 3600, (c / 60) % 60, c % 60)
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, [&'static str; 3]);

    #[test]
    fn test() {
        let tests = [
            TestCase(9, 0, 0, 18, 0, 0, 9, 0, 1, 18, 0, 0, 12, 14, 52, 12, 15, 30, ["9 0 0", "8 59 59", "0 0 38"]),
        ];

        for TestCase(ah1, am1, as1, ah2, am2, as2, bh1, bm1, bs1, bh2, bm2, bs2, ch1, cm1, cs1, ch2, cm2, cs2, expected) in tests {
            assert_eq!(run( ah1, am1, as1, ah2, am2, as2, bh1, bm1, bs1, bh2, bm2, bs2, ch1, cm1, cs1, ch2, cm2, cs2), expected);
        }
    }
}
