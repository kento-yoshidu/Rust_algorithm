// https://atcoder.jp/contests/abc105/tasks/abc105_a

fn run(n: usize, k: usize) -> usize {
	if n % k == 0 {
		0
	} else {
		1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(7, 3, 1),
			TestCase(100, 10, 0),
			TestCase(1, 1, 0),
		];

		for TestCase(n, k, expected) in tests {
			assert_eq!(run(n, k), expected);
		}
	}
}
