// https://atcoder.jp/contests/abc201/tasks/abc201_a

pub fn run(a: Vec<usize>) -> String {
    let mut vec = a.clone();

    vec.sort();

    if vec[2] - vec[1] == vec[1] - vec[0] {
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
        assert_eq!(String::from("Yes"), run(vec![5, 1, 3]));
        assert_eq!(String::from("No"), run(vec![1, 4, 3]));
        assert_eq!(String::from("Yes"), run(vec![5, 5, 5]));
    }
}
