// https://atcoder.jp/contests/abc285/tasks/abc285_b

pub fn run(n: usize, s: &str) -> Vec<usize> {
    let mut ans = Vec::new();
    let vec: Vec<char> = s.chars().collect();

    for i in 1..n {
        for j in 1..=n {
            if i + j > n {
                ans.push(j-1);
                break;
            }

            if vec[j-1] == vec[i+j-1] {
                ans.push(j-1);
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![5, 1, 2, 0, 1], run(6, "abcbac"));
    }
}
