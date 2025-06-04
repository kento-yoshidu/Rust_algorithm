// https://atcoder.jp/contests/abc085/tasks/abc085_d

fn run(_n: usize, h: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut hp = h;

    // 振るダメージの最大値
    let max_a = ab.iter().map(|(a, _)| a).max().unwrap();

    // max_aよりも大きい投げるダメージを収集
    let mut vec: Vec<usize> = ab
        .iter()
        .filter_map(|&(_, b)| if max_a < &b { Some(b) } else { None })
        .collect();

    vec.sort();
    vec.reverse();

    let mut count = 0;

    // 投げるダメージの大きい順にhpを減らしていく
    for b in vec {
        // 投げるダメージのみでhpを0に出来た時
        if b >= hp {
            return count + 1;
        } else {
            hp -= b;
            count += 1;
        }
    }

    // 残りhpを振るダメージで減らす
    if hp % max_a == 0 {
        count + hp / *max_a
    } else {
        count + hp / *max_a + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 10, vec![(3, 5)], 3),
            TestCase(2, 10, vec![(3, 5), (2, 6)], 2),
            TestCase(4, 1000000000, vec![(1, 1), (1, 10000000), (1, 30000000), (1, 99999999)], 860000004),
            TestCase(5, 500, vec![(35, 44), (28, 83), (46, 62), (31, 79), (40, 43)], 9),
        ];

        for TestCase(n, h, ab, expected) in tests {
            assert_eq!(run(n, h, ab), expected);
        }
    }
}
