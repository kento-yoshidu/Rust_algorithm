// https://atcoder.jp/contests/abc200/tasks/abc200_b

pub fn run(n: usize, k: usize) -> usize {
    let mut ans = n.to_string();

    for _ in 0..k {
        let num = ans.parse::<usize>().unwrap();

        if num % 200 == 0 {
            ans = (num / 200).to_string();
        } else {
            ans = ans + "200"
        }
    }

    ans.parse().unwrap()
}

pub fn run2(n: usize, k: usize) -> usize {
    (0..k)
        .fold(n, |state, _| {
            if state % 200 == 0 {
                state / 200
            } else {
                (state.to_string() + "200").parse().unwrap()
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(50531, run(2021, 4));
        assert_eq!(1, run(40000, 2));
        assert_eq!(84875488281, run(8691, 20));
    }

    #[test]
    fn test2() {
        assert_eq!(50531, run2(2021, 4));
        assert_eq!(1, run2(40000, 2));
        assert_eq!(84875488281, run2(8691, 20));
    }
}
