// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/description/

use std::collections::HashSet;

fn run(nums: Vec<i32>) -> usize {
    let mut seen = vec![false; 101];

    for (i, num) in nums.into_iter().enumerate().rev() {
        if seen[num as usize] {
            return (i + 3) / 3;
        }

        seen[num as usize] = true;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<i32>, usize);

    #[test]
    fn leet_3396() {
        let tests = [
            TestCase(vec![1, 2, 3, 4, 2, 3, 3, 5, 7], 2),
            TestCase(vec![4, 5, 6, 4, 4], 2),
            TestCase(vec![6, 7, 8, 9], 0),
        ];

        for TestCase(nums, expected) in tests {
            assert_eq!(run(nums), expected);
        }
    }
}
