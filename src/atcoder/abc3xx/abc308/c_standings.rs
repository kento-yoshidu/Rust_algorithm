// https://atcoder.jp/contests/abc308/tasks/abc308_c

pub fn run(_n: usize, aa: Vec<Vec<usize>>) -> Vec<usize> {
    let mut vec = Vec::new();

    for (i, a) in aa.iter().enumerate() {
        vec.push((i + 1, a[0], a[1]));
    }

    vec.sort_by(|l, r| {
        let (_, la, lb) = l;
        let (_, ra, rb) = r;

        (ra * (la + lb)).cmp(&(la * (ra + rb)))
    });

    vec.iter().map(|(i, _, _)| *i).collect()
}

fn run2(_n: usize, vec: Vec<(i32, i32)>) -> Vec<usize> {
    let mut ans = vec.iter().enumerate().map(|(i, v)| {
        (i+1, (v.0 as f64 / (v.0 + v.1) as f64))
    }).collect::<Vec<(usize, f64)>>();

    // f64はstd::cmp::Ordが実装されておらずcmpが使えないので、partial_cmpを使う
    ans.sort_by(|a, b| (-a.1).partial_cmp(&(-b.1)).unwrap());

    ans.iter().map(|m| {
        m.0
    }).collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 3, 1], run(3, vec![vec![1, 3], vec![3, 1], vec![2, 2]]));
        assert_eq!(vec![1, 2], run(2, vec![vec![1, 3], vec![2, 6]]));
        assert_eq!(vec![3, 1, 4, 2], run(4, vec![vec![999999999, 1000000000], vec![333333333, 999999999], vec![1000000000, 999999997], vec![999999998, 1000000000]]));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![2, 3, 1], run2(3, vec![(1, 3), (3, 1), (2, 2)]));
        assert_eq!(vec![1, 2], run2(2, vec![(1, 3), (2, 6)]));
        assert_eq!(vec![3, 1, 4, 2], run2(4, vec![(999999999, 1000000000), (333333333, 999999999), (1000000000, 999999997), (999999998, 1000000000)]));
    }
}
