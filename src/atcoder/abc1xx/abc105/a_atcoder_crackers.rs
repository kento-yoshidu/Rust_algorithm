// https://atcoder.jp/contests/abc105/tasks/abc105_a

pub fn run(n: usize, k: usize) -> usize {
	n % k
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(1, run(7, 3));
		assert_eq!(0, run(100, 10));
		assert_eq!(0, run(1, 1));
	}
}
