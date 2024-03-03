// https://atcoder.jp/contests/abc343/tasks/abc343_c

fn check(n: &str) -> bool {
    n.chars().eq(n.chars().rev())
}

pub fn run(n: usize) -> usize {
    let mut ans = 1;

    for i in 1..=n {
        if i*i*i > n {
            break;
        }

        if check(&(i*i*i).to_string()) == true {
            ans = ans.max(i*i*i);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Testase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            Testase(345, 343),
            Testase(6, 1),
            Testase(123456789012345, 1334996994331),
        ];

        for Testase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
