// https://atcoder.jp/contests/abc136/tasks/abc136_chttps://atcoder.jp/contests/abc136/tasks/abc136_c

pub fn run(n: usize, h: Vec<isize>) -> String {
    let mut vec = h.clone();

    vec.reverse();

    for i in 0..n-1 {
        if vec[i] + 1 == vec[i+1] {
            vec[i+1] -= 1;
        } else if vec[i+1] - vec[i] >= 2 {
            return String::from("No")
        }
    }

    String::from("Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5, vec![1, 2, 1, 1, 3]));
        assert_eq!(String::from("No"), run(4, vec![1, 3, 2, 1]));
        assert_eq!(String::from("Yes"), run(5, vec![1, 2, 3, 4, 5]));
        assert_eq!(String::from("Yes"), run(1, vec![1000000000]));
        assert_eq!(String::from("No"), run(5, vec![1, 1, 3, 1, 1]));
    }
}
