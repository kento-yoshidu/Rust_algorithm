// https://atcoder.jp/contests/abc320/tasks/abc320_c

pub fn run(n: usize, s: Vec<&str>) -> isize {
    let chars: Vec<_> = s.iter().map(|v| v.chars().collect::<Vec<char>>()).collect();

    let mut ans = std::usize::MAX;

    for i in 0.. n {
        for j in 1..n {
            for k in 1..n {

                for num in 0..3 {
                    let c0 = chars[num][i];
                    let c1 = chars[(num+1)%3][(i+j) % n];
                    let c2 = chars[(num+2)%3][(i+j+k) % n];

                    if c0 == c1 && c1 == c2 {
                        ans = ans.min(i + j + k);
                    }
                }
            }
        }
    }

    if ans == std::usize::MAX {
        -1
    } else {
        ans as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(10, vec!["1937458062", "8124690357", "2385760149"]));
        assert_eq!(20, run(20, vec!["01234567890123456789", "01234567890123456789", "01234567890123456789"]));
        assert_eq!(-1, run(5, vec!["11111", "22222", "33333"]));
    }
}
