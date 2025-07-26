// https://leetcode.com/problems/count-good-triplets/description/

fn run(arr: Vec<isize>, a: isize, b: isize, c: isize) -> usize {
    let mut ans = 0;

    let len = arr.len();

    for i in 0..len {
        for j in i+1..len {
            if (arr[i] - arr[j]).abs() > a {
                continue;
            }

            for k in j+1..len {
                if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                    ans += 1;
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<isize>, isize, isize, isize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![3, 0, 1, 1, 9, 7], 7, 2, 3, 4),
            TestCase(vec![1, 1, 2, 2, 3], 0, 0, 1, 0),
        ];

        for TestCase(arr, a, b, c, expected) in tests {
            assert_eq!(run(arr, a, b, c), expected);
        }
    }
}
