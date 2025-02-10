// https://atcoder.jp/contests/abc123/tasks/abc123_d

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn run(_x: usize, _y: usize, _z: usize, k: usize, a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> Vec<usize> {
    let mut ab = BinaryHeap::new();

    // AとBの和で大きい順にK個求める
    for i in a.iter() {
        for j in b.iter() {
            ab.push(Reverse(i + j));

            if ab.len() > k {
                ab.pop();
            }
        }
    }

    let mut vec: Vec<_> = ab.into_sorted_vec();
    vec.reverse();

    let mut ans = BinaryHeap::new();

    // ABとCの和で大きい順にK個求める
    for i in vec.iter() {
        for j in c.iter() {
            ans.push(Reverse(i.0 + j));

            if ans.len() > k {
                ans.pop();
            }
        }
    }

    ans
        .into_sorted_vec()
        .into_iter()
        .map(|x| x.0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 2, 8, vec![4, 6], vec![1, 5], vec![3, 8], vec![19, 17, 15, 14, 13, 12, 10, 8]),
            TestCase(3, 3, 3, 5, vec![1, 10, 100], vec![2, 20, 200], vec![1, 10, 100], vec![400, 310, 310, 301, 301]),
            TestCase(10, 10, 10, 20, vec![7467038376, 5724769290, 292794712, 2843504496, 3381970101, 8402252870, 249131806, 6310293640, 6690322794, 6082257488], vec![1873977926, 2576529623, 1144842195, 1379118507, 6003234687, 4925540914, 3902539811, 3326692703, 484657758, 2877436338], vec![4975681328, 8974383988, 2882263257,  7690203955, 514305523, 6679823484, 4263279310, 585966808, 3752282379, 620585736], vec![23379871545, 22444657051, 22302177772, 22095691512, 21667941469, 21366963278, 21287912315, 21279176669, 21160477018, 21085311041, 21059876163, 21017997739, 20703329561, 20702387965, 20590247696, 20383761436, 20343962175, 20254073196, 20210218542, 20150096547]),
            ];

        for TestCase(x, y, z, k, a, b, c, expected) in tests {
            assert_eq!(run(x, y, z, k, a, b, c), expected);
        }
    }
}
