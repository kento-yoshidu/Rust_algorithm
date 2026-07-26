// https://atcoder.jp/contests/abc468/tasks/abc468_d

pub fn run(s: &str) -> usize {
    let s: Vec<char> = s.chars().collect();

    let len = s.len() as isize;

    let mut ans: usize = 0;

    for k in 0..2 {
        for st in 0..len {
            let mut l = st - k;
            let mut r = st;
            let mut cnt = 0;

            println!("=== k={k}, st={st} スタート ===");

            while l >= 0 && r < len {
                println!("  比較: s[{l}]='{}' vs s[{r}]='{}'", s[l as usize], s[r as usize]);

                if s[l as usize] != s[r as usize] {
                    cnt += 1;
                    println!("    不一致！ cnt={cnt}");

                    if cnt == 2 {
                        println!("    cnt==2なのでbreak(このl,rはカウントしない)");
                        break;
                    }
                }

                l -= 1;
                r += 1;
                ans += 1;
                println!("    ans+=1 → ans={ans} (l={l}, r={r}に広げて次へ)");
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc468_d() {
        let tests = [
            TestCase("ababa", 13),
            TestCase("atcoder", 18),
            TestCase("abccbacbacb", 40),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}

/*
=== k=0, st=0 スタート ===
  比較: s[0]='a' vs s[0]='a'
    ans+=1 → ans=1 (l=-1, r=1に広げて次へ)
=== k=0, st=1 スタート ===
  比較: s[1]='t' vs s[1]='t'
    ans+=1 → ans=2 (l=0, r=2に広げて次へ)
  比較: s[0]='a' vs s[2]='c'
    不一致！ cnt=1
    ans+=1 → ans=3 (l=-1, r=3に広げて次へ)
=== k=0, st=2 スタート ===
  比較: s[2]='c' vs s[2]='c'
    ans+=1 → ans=4 (l=1, r=3に広げて次へ)
  比較: s[1]='t' vs s[3]='o'
    不一致！ cnt=1
    ans+=1 → ans=5 (l=0, r=4に広げて次へ)
  比較: s[0]='a' vs s[4]='d'
    不一致！ cnt=2
    cnt==2なのでbreak(このl,rはカウントしない)
=== k=0, st=3 スタート ===
  比較: s[3]='o' vs s[3]='o'
    ans+=1 → ans=6 (l=2, r=4に広げて次へ)
  比較: s[2]='c' vs s[4]='d'
    不一致！ cnt=1
    ans+=1 → ans=7 (l=1, r=5に広げて次へ)
  比較: s[1]='t' vs s[5]='e'
    不一致！ cnt=2
    cnt==2なのでbreak(このl,rはカウントしない)
=== k=0, st=4 スタート ===
  比較: s[4]='d' vs s[4]='d'
    ans+=1 → ans=8 (l=3, r=5に広げて次へ)
  比較: s[3]='o' vs s[5]='e'
    不一致！ cnt=1
    ans+=1 → ans=9 (l=2, r=6に広げて次へ)
  比較: s[2]='c' vs s[6]='r'
    不一致！ cnt=2
    cnt==2なのでbreak(このl,rはカウントしない)
=== k=0, st=5 スタート ===
  比較: s[5]='e' vs s[5]='e'
    ans+=1 → ans=10 (l=4, r=6に広げて次へ)
  比較: s[4]='d' vs s[6]='r'
    不一致！ cnt=1
    ans+=1 → ans=11 (l=3, r=7に広げて次へ)
=== k=0, st=6 スタート ===
  比較: s[6]='r' vs s[6]='r'
    ans+=1 → ans=12 (l=5, r=7に広げて次へ)
=== k=1, st=0 スタート ===
=== k=1, st=1 スタート ===
  比較: s[0]='a' vs s[1]='t'
    不一致！ cnt=1
    ans+=1 → ans=13 (l=-1, r=2に広げて次へ)
=== k=1, st=2 スタート ===
  比較: s[1]='t' vs s[2]='c'
    不一致！ cnt=1
    ans+=1 → ans=14 (l=0, r=3に広げて次へ)
  比較: s[0]='a' vs s[3]='o'
    不一致！ cnt=2
    cnt==2なのでbreak(このl,rはカウントしない)
=== k=1, st=3 スタート ===
  比較: s[2]='c' vs s[3]='o'
    不一致！ cnt=1
    ans+=1 → ans=15 (l=1, r=4に広げて次へ)
  比較: s[1]='t' vs s[4]='d'
    不一致！ cnt=2
    cnt==2なのでbreak(このl,rはカウントしない)
=== k=1, st=4 スタート ===
  比較: s[3]='o' vs s[4]='d'
    不一致！ cnt=1
    ans+=1 → ans=16 (l=2, r=5に広げて次へ)
  比較: s[2]='c' vs s[5]='e'
    不一致！ cnt=2
    cnt==2なのでbreak(このl,rはカウントしない)
=== k=1, st=5 スタート ===
  比較: s[4]='d' vs s[5]='e'
    不一致！ cnt=1
    ans+=1 → ans=17 (l=3, r=6に広げて次へ)
  比較: s[3]='o' vs s[6]='r'
    不一致！ cnt=2
    cnt==2なのでbreak(このl,rはカウントしない)
=== k=1, st=6 スタート ===
  比較: s[5]='e' vs s[6]='r'
    不一致！ cnt=1
    ans+=1 → ans=18 (l=4, r=7に広げて次へ)
*/
