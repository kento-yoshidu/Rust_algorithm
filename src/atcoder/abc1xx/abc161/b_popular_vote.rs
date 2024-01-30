// https://atcoder.jp/contests/abc161/tasks/abc161_b

pub fn run(_n: usize, m: usize, a: Vec<usize>) -> String {
    let total: usize = a.iter().sum();
    let border = (total as f64 / (4 * m) as f64).ceil() as usize;

    let count = a.iter()
        .filter(|num| {
            **num >= border
        })
        .count();

    if count >= m {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(4, 1, vec![5, 4, 2, 1]));
        assert_eq!(String::from("No"), run(3, 2, vec![380, 19, 1]));
        assert_eq!(String::from("Yes"), run(12, 3, vec![4, 56, 78, 901, 2, 345, 67, 890, 123, 45, 6, 789]));
    }
}
