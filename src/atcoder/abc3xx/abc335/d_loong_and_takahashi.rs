// https://atcoder.jp/contests/abc335/tasks/abc335_d

pub fn run(n: isize) -> Vec<Vec<String>> {
    let mut ans = vec![vec!["0".to_string(); n as usize]; n as usize];

    let mut pos: (isize, isize) = (0, 0);
    let mut dx: isize = 1;
    let mut dy: isize = 0;

    for i in 1..=n*n {
        if i == n*n {
            ans[pos.1 as usize][pos.0 as usize] = "T".to_string();
        } else {
            ans[pos.1 as usize][pos.0 as usize] = i.to_string();
        }

        if !((0 <= pos.0 + dx && pos.0 + dx < n) && (0 <= pos.1+ dy && pos.1 + dy < n) && ans[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == "0".to_string()) {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
        }

        pos.0 += dx;
        pos.1 += dy;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec!["1", "2", "3", "4", "5"], vec!["16", "17", "18", "19", "6"], vec!["15", "24", "T", "20", "7"], vec!["14", "23", "22", "21", "8"], vec!["13", "12", "11", "10", "9"]], run(5));
    }
}
