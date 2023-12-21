// https://atcoder.jp/contests/abc135/tasks/abc135_b

pub fn run(_n: usize, p: Vec<usize>) -> String {
    let vec = p.clone();
    let mut new_vec = p.clone();

    new_vec.sort();

    let count = vec.iter()
        .zip(new_vec.iter())
        .filter(|t| {
            t.0 != t.1
        })
        .count();

    if count <= 2 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("YES"), run(5, vec![5, 2, 3, 4, 1]));
        assert_eq!(String::from("NO"), run(5, vec![2, 4, 3, 5, 1]));
        assert_eq!(String::from("YES"), run(7, vec![1, 2, 3, 4, 5, 6, 7]));
    }
}
