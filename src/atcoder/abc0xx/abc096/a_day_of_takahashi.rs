// https://atcoder.jp/contests/abc096/tasks/abc096_a

fn run(a: usize, b: usize) -> usize {
	if a <= b {
		a
	} else {
		if a == 1 {
			1
		} else {
			a - 1
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(5, run(5, 5));
		assert_eq!(1, run(2, 1));
		assert_eq!(11, run(11, 30));
		assert_eq!(1, run(1, 1));
	}
}
