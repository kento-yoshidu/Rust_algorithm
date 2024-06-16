// https://atcoder.jp/contests/abc358/tasks/abc358_b

pub fn run(_n: usize, a: usize, t: Vec<usize>) -> Vec<usize> {
    let mut ans = vec![t[0] + a];

    for (i, time) in t.into_iter().enumerate().skip(1) {
        if ans[i-1] >= time {
            ans.push(ans[i-1] + a);
        } else {
            ans.push(time + a);
        }
        println!("{}", time);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, vec![0, 2, 10], vec![4, 8, 14]),
            TestCase(3, 3, vec![1, 4, 7], vec![4, 7, 10]),
            TestCase(10, 50000, vec![120190, 165111, 196897, 456895, 540000, 552614, 561627, 743796, 757613, 991216], vec![170190, 220190, 270190, 506895, 590000, 640000, 690000, 793796, 843796, 1041216]),
        ];

        for TestCase(n, a, t, expected) in tests {
            assert_eq!(run(n, a, t), expected);
        }
    }
}
