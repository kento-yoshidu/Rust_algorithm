// 任意の数が素数であるかどうかを、単純に全探索して求める
pub fn is_prime_a(n: Vec<usize>) -> Vec<bool> {
    n.iter().map(|num| {
        (2..*num).all(|i| {
            num % i != 0
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![true, true, false, true, false, true, false, false, false], is_prime_a(vec![2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true], is_prime_a(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]));
    }
}

/*
pub fn next_prime_number(n: usize) -> usize {
    let res = (n+2..n*2).step_by(2).find(|num| {
        (2..*num/2).all(|i| {
            *num % i != 0
        })
    });

    println!("{:?}", res);

    10
}
*/

