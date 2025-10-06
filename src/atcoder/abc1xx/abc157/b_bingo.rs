// https://atcoder.jp/contests/abc157/tasks/abc157_b

fn run(a: Vec<Vec<usize>>, _n: usize, vec: Vec<usize>) -> &'static str {
    let mut bingo = a.clone();

    // 穴が開いた部分を0に置き換え
    for v in bingo.iter_mut() {
        for b in v.iter_mut() {
            if vec.iter().find(|e| *e == b).is_some() {
                *b = 0
            }
        }
    }

    // 横に空いてるかチェック
    for row in bingo.iter() {
        if row[0] == 0 && row[1] == 0 && row[2] == 0 {
            return "Yes";
        }
    }

    // 縦に空いてるかチェック
    for i in 0..3 {
        if bingo[0][i] == 0 && bingo[1][i] == 0 && bingo[2][i] == 0 {
            return "Yes";
        }
    }

    // 斜めに空いてるか
    if bingo[0][0] == 0 && bingo[1][1] == 0 && bingo[2][2] == 0 {
        return "Yes";
    }
    if bingo[0][2] == 0 && bingo[1][1] == 0 && bingo[2][0] == 0 {
        return "Yes";
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<Vec<usize>>, usize, Vec<usize>, &'static str);

    #[test]
    fn abc157_b() {
        let tests = [
            TestCase(vec![vec![84, 97, 66], vec![79, 89, 11], vec![61, 59, 7]], 7, vec![89, 7, 87, 79, 24, 84, 30], "Yes"),
            TestCase(vec![vec![41, 7, 46], vec![26, 89, 2], vec![78, 92, 8]], 5, vec![6, 45, 16, 57, 17], "No"),
            TestCase(vec![vec![60, 88, 34], vec![92, 41, 43], vec![65, 73, 48]], 10, vec![60, 43, 88, 11, 48, 73, 65, 41, 92, 34], "Yes"),
        ];

        for TestCase(a, n, b, expected) in tests {
            assert_eq!(run(a, n, b), expected);
        }
    }
}
