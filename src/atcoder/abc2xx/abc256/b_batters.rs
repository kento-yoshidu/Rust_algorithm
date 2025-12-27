// https://atcoder.jp/contests/abc256/tasks/abc256_b

fn run(n: usize, a: Vec<usize>) -> usize {
    /* トータルの人数から最後に残っているランナーの人数を引く */

    // 最終的に残るランナーの人数
    let mut rest_runner = 0;

    // 出ているランナーの数
    let mut on_base = 0;

    // vecを後ろから見て行って、累積和が4以上になったらその時点でforループ終了
    // 4以上になった時点でそれまで（配列の前方向）に出ていたランナーは一掃されている
    // 末尾が4の時、ホームランでランナーが一掃されると考えると分かりやすい
    for i in a.into_iter().rev() {
        on_base += i;

        if on_base >= 4 {
            break;
        } else {
            rest_runner += 1;
        }
    }

    n - rest_runner
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc256_b() {
        let tests = [
            TestCase(4, vec![1, 1, 3, 2], 3),
            TestCase(3, vec![1, 1, 1], 0),
            TestCase(10, vec![2, 2, 4, 1, 1, 1, 4, 2, 2, 1], 8),
            TestCase(11, vec![2, 2, 4, 1, 1, 1, 4, 2, 2, 1, 4], 11),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
