// https://atcoder.jp/contests/abc107/tasks/abc107_a

pub fn run(n: usize, i: usize) -> usize {
	n - i + 1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(3, run(4, 2));
		assert_eq!(1, run(1, 1));
		assert_eq!(5, run(15, 11));
	}
}
