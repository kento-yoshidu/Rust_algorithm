// https://atcoder.jp/contests/abc102/tasks/abc102_a

pub fn run(n: usize) -> usize {
	if n % 2 == 0 {
		n
	} else {
		n * 2
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(6, run(3));
		assert_eq!(10, run(10));
		assert_eq!(1999999998, run(999999999));
	}
}
