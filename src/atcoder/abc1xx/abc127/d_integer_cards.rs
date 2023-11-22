// https://atcoder.jp/contests/abc127/tasks/abc127_d

pub fn run(n: usize, _m: usize, a: Vec<usize>, bc: Vec<(usize, usize)>) -> usize {
    let mut bc_vec = bc.clone();

    let mut a_vec: Vec<(usize, usize)> = a.iter()
        .map(|num|
            (1, *num)
        )
        .collect();

    a_vec.append(&mut bc_vec);
    a_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut count = n;
    let mut ans = 0;
    let mut index = 0;

    while 0 < count {
        let ele = a_vec[index];

        if ele.0 >= count {
            ans += count * ele.1;
            break
        } else {
            ans += ele.0 * ele.1;
        }

        count -= ele.0;
        index += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(14, run(3, 2, vec![5, 1, 4], vec![(2, 3), (1, 5)]));
        assert_eq!(338, run(10, 3, vec![1, 8, 5, 7, 100, 4, 52, 33, 13, 5], vec![(3, 10), (4, 30), (1, 4)]));
        assert_eq!(300, run(3, 2, vec![100, 100, 100], vec![(3, 99), (3, 99)]));
        assert_eq!(10000000001, run(11, 3, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![(3, 1000000000), (4, 1000000000), (3, 1000000000)]));
    }
}
