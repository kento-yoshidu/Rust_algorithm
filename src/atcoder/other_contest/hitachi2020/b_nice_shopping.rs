// https://atcoder.jp/contests/hitachi2020/tasks/hitachi2020_b

pub fn run(_a: usize, _b: usize, _m: usize, a_vec: Vec<usize>, b_vec: Vec<usize>, vec: Vec<Vec<usize>>) -> usize {
    // 割引券を使わない場合
    let non_discount = a_vec.iter().min().unwrap() + b_vec.iter().min().unwrap();

    // 割引券を使う場合
    let mut discount = 100;

    for v in vec.iter() {
        discount = discount.min(a_vec[v[0]-1] + b_vec[v[1]-1] - v[2]);
    }

    if non_discount > discount {
        discount
    } else {
        non_discount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(2, 3, 1, vec![3, 3], vec![3, 3, 3], vec![vec![1, 2, 1]]));
        assert_eq!(10, run(1, 1, 2, vec![10], vec![10], vec![vec![1, 1, 5], vec![1, 1, 10]]));
        assert_eq!(6, run(2, 2, 1, vec![3, 5], vec![3, 5], vec![vec![2, 2, 2]]));
    }
}
