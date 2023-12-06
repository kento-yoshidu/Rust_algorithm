// https://atcoder.jp/contests/abc292/tasks/abc292_b

pub fn run(n: usize, _q: usize, e: Vec<(usize, usize)>) -> Vec<String> {
    let mut ans = Vec::<String>::new();

    let mut state: Vec<(usize, usize)> = (0..n)
        .map(|i| {
            (i, 0)
        })
        .collect();

    for t in e {
        match t.0 {
            1 => state[t.1 - 1].1 += 1,
            2 => state[t.1 - 1].1 += 2,
            3 => {
                if state[t.1 - 1].1 >= 2 {
                    ans.push(String::from("Yes"))
                } else {
                    ans.push(String::from("No"))
                }
            }
            _ => unreachable!()
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("No"), String::from("No"), String::from("Yes"), String::from("No"), String::from("Yes"), String::from("No")], run(3, 9, vec![(3, 1), (3, 2), (1, 2), (2, 1), (3, 1), (3, 2), (1, 2), (3, 2), (3, 3)]));
        assert_eq!(vec![String::from("No"), String::from("No"), String::from("Yes"), String::from("Yes"), String::from("No")], run(5, 11, vec![(3, 1), (1, 2), (3, 2), (1, 3), (1, 3), (3, 3), (1, 4), (2, 4), (3, 4), (1, 5), (3, 5)]));
    }
}
