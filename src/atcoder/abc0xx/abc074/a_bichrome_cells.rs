// https://atcoder.jp/contests/abc074/tasks/abc074_a

fn run(n: usize, a: usize) -> usize {
	n * n - a
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(5, run(3, 4));
		assert_eq!(261, run(19, 100));
		assert_eq!(100, run(10, 0));
	}
}
