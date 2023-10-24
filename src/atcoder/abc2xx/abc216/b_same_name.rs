// https://atcoder.jp/contests/abc216/tasks/abc216_b

use itertools::Itertools;

pub fn run(_n: usize, v: Vec<(&str, &str)>) -> String {
    if v.iter().permutations(2).any(|arr| {
        arr[0].0 == arr[1].0 && arr[0].1 == arr[1].1
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
        assert_eq!(String::from("Yes"), run(3, vec![("tanaka", "taro"), ("sato", "hanako"), ("tanaka", "taro")]));
        assert_eq!(String::from("No"), run(3, vec![("saito", "ichiro"), ("saito", "jiro"), ("saito", "saburo")]));
        assert_eq!(String::from("No"), run(4, vec![("sypdgido", "bkseq"), ("bajsqz", "hh"), ("ozjekw", "mcybmtt"), ("qfeysvw", "dbo")]));
    }
}
