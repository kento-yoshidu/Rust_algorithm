// https://atcoder.jp/contests/caddi2018b/tasks/caddi2018b_b

pub fn run(_n: usize, h: usize, w: usize, ab: Vec<(usize, usize)>) -> usize {
    ab.iter()
        .filter(|(a, b)| {
            *a >= h && *b >= w
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, 2, vec![(10, 3), (5, 2), (2, 5)], 2),
            TestCase(10, 587586158, 185430194, vec![(894597290, 708587790), (680395892, 306946994), (590262034, 785368612), (922328576, 106880540), (847058850, 326169610), (936315062, 193149191), (702035777, 223363392), (11672949, 146832978), (779291680, 334178158), (615808191, 701464268)], 8),
        ];

        for TestCase(n, h, w, ab, expected) in tests {
            assert_eq!(run(n, h, w, ab), expected);
        }
    }
}
