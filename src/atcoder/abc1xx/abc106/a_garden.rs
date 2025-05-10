// https://atcoder.jp/contests/abc106/tasks/abc106_a

fn run(a: usize, b: usize) -> usize {
	(a - 1) * (b - 1)
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(2, 2, 1),
			TestCase(5, 7, 24),
		];

		for TestCase(a, b, expected) in tests {
			assert_eq!(run(a, b), expected);
		}
	}
}
