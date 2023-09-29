// https://atcoder.jp/contests/abc089/tasks/abc089_a

fn run(n: usize) -> usize {
	n / 3
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(2, run(8));
		assert_eq!(0, run(2));
		assert_eq!(3, run(9));
	}
}
