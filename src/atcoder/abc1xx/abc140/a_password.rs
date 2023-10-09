// https://atcoder.jp/contests/abc140/tasks/abc140_a

pub fn run(n: usize) -> usize {
	n * n * n
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(8, run(2));
		assert_eq!(1, run(1));
		assert_eq!(27, run(3));
	}
}
