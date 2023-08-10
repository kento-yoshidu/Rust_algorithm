// https://atcoder.jp/contests/abc254/tasks/abc254_a

pub fn run(vec: Vec<usize>) -> String {
    if (vec[0] <= vec[1] && vec[1] <= vec[2]) || (vec[0] >= vec[1] && vec[1] >= vec[2]) {
        return String::from("Yes");
    }

    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(vec![5, 3, 2]));
        assert_eq!(String::from("No"), run(vec![2, 5, 3]));
        assert_eq!(String::from("Yes"), run(vec![100, 100, 100]));
    }
}
