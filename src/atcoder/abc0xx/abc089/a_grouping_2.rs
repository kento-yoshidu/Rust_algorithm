// https://atcoder.jp/contests/abc089/tasks/abc089_a

fn run(n: usize) -> usize {
	n / 3
}

#[cfg(test)]
mod test {
	use super::*;

	struct TestCase(usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(8, 2),
			TestCase(2, 0),
			TestCase(9, 3),
		];

		for TestCase(n, expected) in tests {
			assert_eq!(run(n), expected);
		}
	}
}
