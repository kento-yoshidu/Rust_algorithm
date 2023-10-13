// https://atcoder.jp/contests/abc038/tasks/abc038_b

pub fn run(a: Vec<usize>, b: Vec<usize>) -> String {
    if a.iter().any(|a_num| {
        b.iter().any(|b_num| {
            a_num == b_num
        })
    }) {
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
        assert_eq!(String::from("Yes"), run(vec![1080, 1920], vec![1080, 1920]));
        assert_eq!(String::from("Yes"), run(vec![1080, 1920], vec![1920, 1080]));
        assert_eq!(String::from("No"), run(vec![334, 668], vec![669, 1002]));
        assert_eq!(String::from("No"), run(vec![100, 200], vec![300, 150]));
        assert_eq!(String::from("No"), run(vec![120, 120], vec![240, 240]));
    }
}
