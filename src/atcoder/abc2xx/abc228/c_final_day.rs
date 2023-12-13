// https://atcoder.jp/contests/abc228/tasks/abc228_c

fn run(n: usize, k: usize, p: Vec<Vec<usize>>) -> Vec<String> {
    let vec: Vec<usize> = p.iter()
        .map(|t| t.iter().sum())
        .collect();

    let mut vec_clone = vec.clone();
    vec_clone.sort();

    vec.iter()
        .map(|num| {
            if num + 300 > vec_clone[n-k-1] {
                String::from("Yes")
            } else {
                String::from("No")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("Yes"), String::from("Yes"), String::from("No")], run(3, 1, vec![vec![178, 205, 132], vec![112, 220, 96], vec![36, 64, 20]]));
        assert_eq!(vec![String::from("Yes"), String::from("Yes")], run(2, 1, vec![vec![300, 300, 300], vec![200, 200, 200]]));
        assert_eq!(vec![String::from("Yes"), String::from("Yes"), String::from("No"), String::from("Yes")], run(4, 2, vec![vec![127, 235, 78], vec![192, 134, 298], vec![28, 56, 42], vec![96, 120, 250]]));
    }
}
