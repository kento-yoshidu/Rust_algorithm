// https://atcoder.jp/contests/abc139/tasks/abc139_a

pub fn run(s: String, t: String) -> usize {
	let vec_s: Vec<char> = s.chars().collect();
	let vec_t: Vec<char> = t.chars().collect();

	vec_s.iter().zip(vec_t).filter(|v| {
		*v.0 == v.1
	}).count()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(2, run(String::from("CSS"), String::from("CSR")));
		assert_eq!(3, run(String::from("SSR"), String::from("SSR")));
		assert_eq!(0, run(String::from("RRR"), String::from("SSS")));
	}
}
