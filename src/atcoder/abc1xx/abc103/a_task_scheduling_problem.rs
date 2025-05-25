// https://atcoder.jp/contests/abc103/tasks/abc103_a

fn run(a: &Vec<usize>) -> usize {
	let mut vec = a.clone();

	vec.sort();

	vec.windows(2)
		.map(|v| (v[0] as isize - v[1] as isize).abs() as usize)
		.sum()
}

fn run2(a: &Vec<usize>) -> usize {
	a.iter().max().unwrap() - a.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(Vec<usize>, usize);

	#[test]
	fn abc103_a() {
		let tests = [
			TestCase(vec![1, 6, 3], 5),
			TestCase(vec![11, 5, 5], 6),
			TestCase(vec![100, 100, 100], 0)
		];

		for TestCase(a, expected) in tests {
			assert_eq!(run(&a), expected);
			assert_eq!(run2(&a), expected);
		}
	}
}
