// https://atcoder.jp/contests/abc074/tasks/abc074_a

fn run(n: usize, a: usize) -> usize {
	n * n - a
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(3, 4, 5),
			TestCase(19, 100, 261),
			TestCase(10, 0, 100),
		];

		for TestCase(n, a, expected) in tests {
			assert_eq!(run(n, a), expected);
		}
	}
}
