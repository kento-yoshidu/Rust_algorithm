// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cj

fn calc(n: usize, a: &Vec<usize>, x: usize) -> usize {
    let mut vec = a.to_vec();

    vec.sort();

    let mut left = 0;
    let mut right = n;

    while left + 1 < right {
        let middle = (left + right) / 2;

        if x <= vec[middle] {
            right = middle
        } else {
            left = middle
        }
    }

    right
}

pub fn run(n: usize, a: Vec<usize>, _q: usize, x: Vec<usize>) -> Vec<usize> {
    x.iter().map(|num| {
        calc(n, &a, *num)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 4, 5, 8, 10], run(15, vec![83, 31, 11, 17, 32, 19, 23, 37, 43, 47, 53, 61, 67, 5, 55], 5, vec![10, 20, 30, 40, 50]));
        assert_eq!(vec![5, 2], run(5, vec![1, 3, 3, 3, 1], 2, vec![4, 3]));
    }
}
