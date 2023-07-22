// https://atcoder.jp/contests/abc280/tasks/abc280_b

#[allow(dead_code, unused)]
pub fn run(n: isize, vec: Vec<isize>) -> Vec<isize> {
    let mut total = vec![vec[0]];

    for i in 0..vec.len()-1 {
        total.push(vec[i+1] - vec[i]);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 1, 4], run(3, vec![3, 4, 8]));
        assert_eq!(vec![314159265, 44820058, 487285015, -1174214626, 747667227, -1357227521, 1035005041, 133287181, 397839259, -1491424381], run(10, vec![314159265, 358979323, 846264338, -327950288, 419716939, -937510582, 97494459, 230781640, 628620899, -862803482]));
    }
}
