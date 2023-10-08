// https://atcoder.jp/contests/abc280/tasks/abc280_b

pub fn run(_n: isize, vec: Vec<isize>) -> Vec<isize> {
    let mut total = vec![vec[0]];

    for i in 0..vec.len()-1 {
        total.push(vec[i+1] - vec[i]);
    }

    total
}

// 累積和で
fn run2(n: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::<i32>::new();

    // Refactoring
    for (_i, num) in n.iter().enumerate() {
        if ans.len() == 0 {
            ans.push(*num);
            continue;
        }

        let total: i32 = ans.iter().sum();

        ans.push(num - total);
    }

    ans
}

// windowsで
fn run3(_n: usize, vec: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![vec[0]];

    for v in vec.windows(2) {
        ans.push(v[1] - v[0]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 1, 4], run(3, vec![3, 4, 8]));
        assert_eq!(vec![314159265, 44820058, 487285015, -1174214626, 747667227, -1357227521, 1035005041, 133287181, 397839259, -1491424381], run(10, vec![314159265, 358979323, 846264338, -327950288, 419716939, -937510582, 97494459, 230781640, 628620899, -862803482]));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![3, 1, 4], run2(vec![3, 4, 8]));
        assert_eq!(vec![314159265, 44820058, 487285015, -1174214626, 747667227, -1357227521, 1035005041, 133287181, 397839259, -1491424381], run2(vec![314159265, 358979323, 846264338, -327950288, 419716939, -937510582, 97494459, 230781640, 628620899, -862803482]))
    }

    #[test]
    fn test3() {
        assert_eq!(vec![3, 1, 4], run3(3, vec![3, 4, 8]));
        assert_eq!(vec![314159265, 44820058, 487285015, -1174214626, 747667227, -1357227521, 1035005041, 133287181, 397839259, -1491424381], run3(10, vec![314159265, 358979323, 846264338, -327950288, 419716939, -937510582, 97494459, 230781640, 628620899, -862803482]))
    }
}
