// https://atcoder.jp/contests/abc102/tasks/abc102_a

fn run(n: usize) -> usize {
	if n % 2 == 0 {
		n
	} else {
		n * 2
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(3, 6),
			TestCase(10, 10),
			TestCase(999999999, 1999999998),
		];

		for TestCase(n, expected) in tests {
			assert_eq!(run(n), expected);
		}
	}
}
