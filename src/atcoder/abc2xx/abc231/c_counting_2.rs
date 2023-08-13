fn calc(n: usize, num: usize, a: &Vec<usize>) -> usize {
    let mut vec = a.clone();

    vec.sort();

    let mut left = 0;
    let mut right = n;

    loop {
        let middle = (left + right) / 2;

        if left == right || num == vec[middle] {
            return n - middle;
        }

        if num <= vec[middle] {
            right = middle
        } else {
            left = middle + 1
        }
    }
}

pub fn run(n: usize, _q: usize, a: Vec<usize>, x: Vec<usize>) -> Vec<usize> {
    x.iter().map(|num| {
        calc(n, *num, &a)
    }).collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2], run(3, 1, vec![100, 160, 130], vec![120]));
        assert_eq!(vec![0, 1, 2, 3, 4], run(5, 5, vec![1, 2, 3, 4, 5], vec![6, 5, 4, 3, 2]));
        assert_eq!(vec![5, 3, 5, 5, 5], run(5, 5, vec![804289384, 846930887, 681692778, 714636916, 957747794], vec![ 424238336, 719885387, 649760493, 596516650, 189641422]));
    }
}
