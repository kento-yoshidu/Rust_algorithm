// https://leetcode.com/problems/single-number/description/

use std::collections::HashMap;

fn run(nums: &Vec<isize>) -> isize {
    let mut hash_map = HashMap::new();

    for n in nums {
        *hash_map.entry(n).or_insert(0) += 1;
    }

    *hash_map
        .into_iter()
        .find(|&(_, count)| count == 1)
        .map(|(num, _)| num)
        .unwrap()
}

fn run2(nums: &Vec<isize>) -> isize {
    nums.into_iter().fold(0, |acc, num| acc ^ num)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![2, 2, 1], 1),
            TestCase(vec![4, 1, 2, 1, 2], 4),
            TestCase(vec![1], 1),
        ];

        for TestCase(nums, expected) in tests {
            assert_eq!(run(&nums), expected);
            assert_eq!(run2(&nums), expected);
        }
    }
}