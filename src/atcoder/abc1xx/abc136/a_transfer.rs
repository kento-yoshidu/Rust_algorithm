// https://atcoder.jp/contests/abc136/tasks/abc136_a

fn run(a: usize, b: usize, c: usize) -> usize {
	let rest = a - b;

	if rest >= c {
		0
	} else {
		c - rest
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize, usize);

	#[test]
	fn abc136_a() {
		let tests = [
			TestCase(6, 4, 3, 1),
			TestCase(8, 3, 9, 4),
			TestCase(12, 3, 7, 0),
		];

		for TestCase(a, b, c, expected) in tests {
			assert_eq!(run(a, b, c), expected);
		}
	}
}
