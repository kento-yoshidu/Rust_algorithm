// https://atcoder.jp/contests/abc327/tasks/abc327_c

use itertools::Itertools;

fn run(vec: Vec<Vec<usize>>) -> String {
    // 横方向に重複がないか
    for v in vec.iter() {
        if !v.iter().all_unique() {
            return String::from("No")
        }
    }

    // 縦方向に重複がないか
    for i in 0..9 {
        let mut col = Vec::new();

        for j in 0..9 {
            col.push(vec[j][i]);
        }

        if !col.iter().all_unique() {
            return String::from("No")
        }
    }

    // 3x3に重複がないか
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            let mut square: Vec<usize> = Vec::new();

            for ti in 0..3 {
                for tj in 0..3 {
                    square.push(vec[i + ti][j + tj]);
                }
            }

            if !square.iter().all_unique() {
                return String::from("No");
            }
        }
    }

    String::from("Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![2, 3, 4, 5, 6, 7, 8, 9, 1], vec![5, 6, 7, 8, 9, 1, 2, 3, 4], vec![8, 9, 1, 2, 3, 4, 5, 6, 7], vec![3, 4, 5, 6, 7, 8, 9, 1, 2], vec![6, 7, 8, 9, 1, 2, 3, 4, 5], vec![9, 1, 2, 3, 4, 5, 6, 7, 8]]));
        assert_eq!(String::from("No"), run(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![2, 3, 4, 5, 6, 7, 8, 9, 1], vec![3, 4, 5, 6, 7, 8, 9, 1, 2], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![5, 6, 7, 8, 9, 1, 2, 3, 4], vec![6, 7, 8, 9, 1, 2, 3, 4, 5], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![8, 9, 1, 2, 3, 4, 5, 6, 7], vec![9, 1, 2, 3, 4, 5, 6, 7, 8]]));
        assert_eq!(String::from("No"), run(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6]]));
    }
}
