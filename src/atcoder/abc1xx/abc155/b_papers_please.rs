// https://atcoder.jp/contests/abc155/tasks/abc155_b

fn run(_n: usize, a: Vec<usize>) -> &'static str {
	if a.into_iter()
        .filter(|x| x % 2 == 0)
        .all(|x| x % 3 == 0 || x % 5 == 0) {
		"APPROVED"
	} else {
		"DENIED"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, Vec<usize>, &'static str);

	#[test]
	fn abc155_b() {
		let tests = [
			TestCase(5, vec![6, 7, 9, 10, 31], "APPROVED"),
			TestCase(3, vec![28, 27, 24], "DENIED"),
		];

		for TestCase(n, a, expected) in tests {
			assert_eq!(run(n, a), expected);
		}
	}
}
