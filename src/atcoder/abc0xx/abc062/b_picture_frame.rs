// https://atcoder.jp/contests/abc062/tasks/abc062_b

pub fn run(h: usize, w: usize, a: Vec<&str>) -> Vec<String> {
    (0..h+2)
        .enumerate()
        .map(|(i, _)| {
            if i == 0 || i == h+1 {
                "#".to_string().repeat(w+2)
            } else {
                format!("#{}#", &a[i-1])
            }
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("#####"), String::from("#abc#"), String::from("#arc#"), String::from("#####")], run(2, 3, vec!["abc", "arc"]));
        assert_eq!(vec![String::from("###"), String::from("#z#"), String::from("###")], run(1, 1, vec!["z"]));
    }
}
