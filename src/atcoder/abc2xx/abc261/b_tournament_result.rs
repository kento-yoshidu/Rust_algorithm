// https://atcoder.jp/contests/abc261/tasks/abc261_b

pub fn run(n: usize, a: Vec<&str>) -> String {
    let vec: Vec<Vec<char>> = a.iter().map(|a| a.chars().collect()).collect();

    for i in 0..n {
        for j in 0..n {
            if i == j { continue };

            if vec[i][j] == 'W' && vec[j][i] != 'L' {
                return String::from("incorrect");
            } else if vec[i][j] == 'L' && vec[j][i] != 'W' {
                return  String::from("incorrect");
            } else if vec[i][j] == 'D' && vec[j][i] != 'D' {
                return String::from("incorrect");
            }
        }
    }

    String::from("correct")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("incorrect"), run(4, vec!["-WWW", "L-DD", "LD-W", "LDW-"]));
        assert_eq!(String::from("correct"), run(2, vec!["-D", "D-"]));
    }
}