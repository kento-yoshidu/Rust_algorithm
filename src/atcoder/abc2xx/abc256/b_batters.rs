#[allow(dead_code)]
pub fn run(n: i32, vec: Vec<i32>) -> i32 {
    /* トータルの人数から最後に残っているランナーの人数を引く */

    // 最終的に残るランナーの人数
    let mut rest_runner = 0;

    // 出ているランナーの数
    let mut on_base = 0;

    // vecを後ろから見て行って、累積和が4以上になったらその時点でforループ終了
    // 4以上になった時点でそれまで（配列の前方向）に出ていたランナーは一掃されている
    // 末尾が4の時、ホームランでランナーが一掃されると考えると分かりやすい
    for i in vec.iter().rev() {
        on_base += i;

        if on_base >= 4 {
            break
        } else {
            rest_runner += 1;
        }
    }

    n - rest_runner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, vec![1, 1, 3, 2]));
        assert_eq!(0, run(3, vec![1, 1, 1]));
        assert_eq!(8, run(10, vec![2, 2, 4, 1, 1, 1, 4, 2, 2, 1]));
        assert_eq!(11, run(11, vec![2, 2, 4, 1, 1, 1, 4, 2, 2, 1, 4]));
    }
}
