// https://atcoder.jp/contests/abc185/tasks/abc185_d

pub fn run(n: usize, m: usize, a: Option<Vec<usize>>) -> usize {
    if m == 0 {
        return 1;
    }

    let mut vec = vec![0; n+2];
    vec[0] = 1;
    vec[n+1] = 1;

    for n in a.unwrap() {
        vec[n] = 1;
    }

    let chunks: Vec<usize> = vec
        .split(|&x| x == 1) // `1` でベクターを分割する
        .filter(|chunk| !chunk.is_empty()) // 空のスライスを除外
        .map(|chunk| chunk.len()) // 各チャンクの長さを取得
        .collect(); // Vec に収集

    if chunks.len() == 0 {
        return 0;
    }

    let k = chunks.iter().min().unwrap();

    let mut ans = 0;

    for len in chunks.iter() {
        ans += (len + k - 1) / k;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<usize>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, Some(vec![1, 3]), 3),
            TestCase(13, 3, Some(vec![13, 3, 9]), 6),
            TestCase(5, 5, Some(vec![5, 2, 1, 4, 3]), 0),
            TestCase(1, 0, None, 1),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
