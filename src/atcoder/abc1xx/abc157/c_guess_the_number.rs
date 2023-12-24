// https://atcoder.jp/contests/abc157/tasks/abc157_c

pub fn run(n: usize, _m: usize, sc: Vec<(usize, usize)>) -> isize {
    let vec: Vec<(&usize, char)> = sc.iter()
        .map(|(l, c)| {
            (l, char::from_digit(*c as u32, 10).unwrap())
        })
        .collect();

    for i in 0..=999 {
        // 桁が違ったら進める
        if i.to_string().len() < n {
            continue;
        }

        let string: &str = &i.to_string();

        if vec.iter()
            .all(|&(s, t)| {
                string.chars().nth(s-1).unwrap() == t
            }) {
                return i
            }
        }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(702, run(3, 3, vec![(1, 7), (3, 2), (1, 7)]));
        assert_eq!(-1, run(3, 2, vec![(2, 1), (2, 3)]));
        assert_eq!(-1, run(3, 1, vec![(1, 0)]));
    }
}
