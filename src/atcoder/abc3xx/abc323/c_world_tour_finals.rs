// https://atcoder.jp/contests/abc323/tasks/abc323_c

fn run(_n: usize, _m: usize, a: Vec<usize>, s: Vec<&str>) -> Vec<usize> {
    // 現時点でのスコアを集計
    let scores = s.iter().enumerate().map(|(index, v)| {
        v.chars().enumerate().map(|(index, c)| {
            if c == 'o' {
                a[index]
            } else {
                0
            }
        }).sum::<usize>() + index + 1
    }).collect::<Vec<usize>>();

    let mut ans: Vec<usize> = Vec::new();
    // 現時点での最高得点
    let max = scores.iter().max().unwrap();

    for (index, score) in scores.iter().enumerate() {
        // 最高得点を超えていたら即0を返してcontinue
        if score >= max {
            ans.push(0);
            continue
        }

        // まだ解いてない問題のindexを抜き出す
        let mut mondai_index = s[index]
            .chars()
            .enumerate()
            .filter(|(_, c)| {
                *c == 'x'
            })
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        //　得点が高い問題順にindexを並べる
        mondai_index.reverse();

        // 現時点のスコア
        // これがmaxを超えるまでプラスする回数が答え
        let mut current_score = *score;
        let mut count = 0;

        for i in &mondai_index {
            current_score += a[*i];
            count += 1;

            if current_score >= *max {
                ans.push(count);
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1, 1], run(4, 3, vec![1000, 500, 700, 2000], vec!["xxxo", "ooxx", "oxox"]));
        assert_eq!(vec![1, 1, 1, 1, 0], run(5, 5, vec![1000, 1500, 2000, 2000, 2500], vec!["xxxxx", "oxxxx", "xxxxx", "oxxxx", "oxxxx"]));
        assert_eq!(vec![7, 6, 5, 4, 3, 2, 0], run(7, 8, vec![500, 500, 500, 500, 500, 500, 500, 500], vec!["xxxxxxxx", "oxxxxxxx", "ooxxxxxx", "oooxxxxx", "ooooxxxx", "oooooxxx", "ooooooxx"]));
    }
}
