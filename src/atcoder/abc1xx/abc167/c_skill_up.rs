// https://atcoder.jp/contests/abc167/tasks/abc167_c

#[allow(dead_code)]
fn run(n: usize, m: usize, x: usize, vec: Vec<Vec<usize>>) -> i32 {
    let mut ans = std::usize::MAX;

    let mut flag = false;

    for bit in 1..(1 << n) {
        let mut tmp = vec![0; m+1];

        for i in 0..n {
            if bit & (1 << i) != 0 {
                for j in 0..=m {
                    tmp[j] += vec[i][j];
                }
            }
        }

        for i in 1..=m {
            if tmp[i] < x {
                flag = true;
                break;
            }

            ans = ans.min(tmp[0]);
        }
    }

    if flag == true {
        ans as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(120, run(3, 3, 10, vec![vec![60, 2, 2, 4], vec![70, 8, 7, 9], vec![50, 2, 3, 9]]));
        assert_eq!(-1, run(3, 3, 10, vec![vec![100, 3, 1, 4], vec![100, 1, 5, 9], vec![100, 2, 6, 5]]));
        assert_eq!(1067, run(8, 5, 22, vec![vec![100, 3, 7, 5, 3, 1], vec![164, 4, 5, 2, 7, 8], vec![334, 7, 2, 7, 2, 9], vec![234, 4, 7, 2, 8, 2], vec![541, 5, 4, 3, 3, 6], vec![235, 4, 8, 6, 9, 7], vec![394, 3, 6, 1, 6, 2], vec![872, 8, 4, 3, 7, 2]]));
    }
}
