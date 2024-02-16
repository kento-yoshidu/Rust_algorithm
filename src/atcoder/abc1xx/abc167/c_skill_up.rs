// https://atcoder.jp/contests/abc167/tasks/abc167_c

pub fn run(n: usize, m: usize, x: isize, vec: Vec<Vec<isize>>) -> isize {
    let mut ans = std::isize::MAX;

    for bit in 1..(1 << n) {
        let mut temp = vec![0; m];
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += vec[i][0];

                for j in 0..m {
                    temp[j] += vec[i][j+1];
                }
            }
        }

        if temp.iter()
            .all(|num| *num >= x) {
                ans = ans.min(sum);
            }
    }

    if ans != std::isize::MAX {
        ans
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, isize, Vec<Vec<isize>>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, 10, vec![vec![60, 2, 2, 4], vec![70, 8, 7, 9], vec![50, 2, 3, 9]], 120),
            TestCase(3, 3, 10, vec![vec![100, 3, 1, 4], vec![100, 1, 5, 9], vec![100, 2, 6, 5]], -1),
            TestCase(8, 5, 22, vec![vec![100, 3, 7, 5, 3, 1], vec![164, 4, 5, 2, 7, 8], vec![334, 7, 2, 7, 2, 9], vec![234, 4, 7, 2, 8, 2], vec![541, 5, 4, 3, 3, 6], vec![235, 4, 8, 6, 9, 7], vec![394, 3, 6, 1, 6, 2], vec![872, 8, 4, 3, 7, 2]], 1067),
            TestCase(1, 1, 100000, vec![vec![100000, 100000]], 100000),
            TestCase(12, 1, 12345, vec![vec![10846, 12345], vec![2044, 10000], vec![8800, 2345], vec![9385, 1350], vec![91370, 13824], vec![20387, 20967], vec![24034, 24807], vec![12459, 49024], vec![4805, 9999], vec![1305, 1], vec![32495, 39566], vec![15903, 8883]], 6849),
            TestCase(2, 11, 36426, vec![vec![79445, 5711, 47765, 32760, 90408, 24492, 41078, 36756, 68794, 2060, 62118, 92121], vec![2156, 12685, 18891, 59613, 23256, 26016, 46755, 56694, 97455, 85238, 49611, 95092]], -1),
        ];

        for TestCase(n, m, x, vec, expected) in tests {
            assert_eq!(run(n, m, x, vec), expected);
        }
    }
}
