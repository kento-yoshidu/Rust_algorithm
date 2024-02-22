// https://atcoder.jp/contests/abc225/tasks/abc225_c

pub fn run(n: usize, m: usize, vec: Vec<Vec<isize>>) -> &'static str {
    // 右方向に+1以上されていないか
    for v in vec.iter() {
        for i in 0..m-1 {
            if (v[i+1] - v[i]).abs() > 1 {
                return "No";
            }
        }
    }

    // 縦方向に+7されているか
    for i in 0..n-1 {
        for j in 0..m {
            if vec[i][j] + 7 != vec[i+1][j] {
                return "No";
            }
        }
    }

    // 右端以外の値が7で割り切れたらNoを返す
    for v in vec.iter() {
        for i in 0..m-1 {
            if v[i] % 7 == 0 {
                return "No";
            }
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<isize>>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec![vec![1, 2, 3], vec![8, 9, 10], vec![15, 16, 17]], "Yes"),
            TestCase(2, 1, vec![vec![1], vec![2]], "No"),
            TestCase(2, 3, vec![vec![7, 8, 9], vec![14, 15, 16]], "No"),
            TestCase(2, 3, vec![vec![5, 6, 7], vec![12, 13, 14]], "Yes"),
            TestCase(10, 4, vec![vec![1346, 1347, 1348, 1349],vec![1353, 1354, 1355, 1356], vec![1360, 1361, 1362, 1363], vec![1367, 1368, 1369, 1370], vec![1374, 1375, 1376, 1377], vec![1381, 1382, 1383, 1384], vec![1388, 1389, 1390, 1391], vec![1395, 1396, 1397, 1398], vec![1402, 1403, 1404, 1405], vec![1409, 1410, 1411, 1412]], "Yes"),
            TestCase(2, 2, vec![vec![1, 9], vec![8, 16]], "No"),
            TestCase(1, 7, vec![vec![789335564, 749650823, 914426362, 880284765, 939766774, 688515116, 148723218]], "No"),
        ];

        for TestCase(n, m, vec, expected) in tests {
            assert_eq!(run(n, m, vec), expected);
        }
    }
}
