// https://atcoder.jp/contests/abc110/tasks/abc110_b

pub fn run(_n: isize, _m: isize, x: isize, y: isize, x_vec: Vec<isize>, y_vec: Vec<isize>) -> String {
    let x_max = x_vec.iter().max().unwrap();
    let y_min = y_vec.iter().min().unwrap();

    if (x+1..=y)
        .any(|num| {
            x_max < &num && &num <= y_min
        }) {
            String::from("No War")
        } else {
            String::from("War")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("No War"), run(3, 2, 10, 20, vec![8, 15, 13], vec![16, 22]));
        assert_eq!(String::from("War"), run(4, 2, -48, -1, vec![-20, -35, -91, -23], vec![-22, 66]));
        assert_eq!(String::from("War"), run(5, 3, 6, 8, vec![-10, 3, 1, 5, -100], vec![100, 6, 14]));
    }
}
