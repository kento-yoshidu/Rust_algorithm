// https://atcoder.jp/contests/abc302/tasks/abc302_b

pub fn run(h: usize, w: usize, s: Vec<&str>) -> Vec<(i32, i32)> {
    let vec: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    // 下、右、右下、左下
    let dy = vec![1, 0, 1, 1];
    let dx = vec![0, 1, 1, -1];

    for i in 0..h {
        for j in 0..w {
            for k in 0..4 {
                let mut current = (i as i32, j as i32);
                let mut temp = Vec::new();
                let mut pos = Vec::new();

                for _ in 0..5 {
                    pos.push((current.0, current.1));
                    temp.push(vec[current.0 as usize][current.1 as usize]);

                    // 端っこを超えたらやめる
                    if current.0 + dy[k] < 0 || current.0 + dy[k] >= h as i32 {
                        continue;
                    }

                    // 端っこを超えたらやめる
                    if current.1 + dx[k] < 0 || current.1 + dx[k] >= w as i32 {
                        continue;
                    }

                    current.0 = (current.0 + dy[k] + h as i32) % h as i32;
                    current.1 = (current.1 + dx[k] + w as i32) % w as i32;
                }

                let str: String = temp.iter().collect();
                let str_rev: String = temp.iter().rev().collect();

                if str == "snuke" {
                    let ans: Vec<(i32, i32)> = pos.iter().map(|(i, j)| (i+1, j+1)).collect();

                    return ans;
                }

                if str_rev == "snuke" {
                    let ans: Vec<(i32, i32)> = pos.iter().rev().map(|(i, j)| (i+1, j+1)).collect();

                    return ans;
                }
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<(i32, i32)>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 6, vec!["vgxgpu", "amkxks", "zhkbpp", "hykink", "esnuke", "zplvfj"], vec![(5, 2), (5, 3), (5, 4), (5, 5), (5, 6)]),
            TestCase(5, 5, vec!["ezzzz", "zkzzz", "ezuzs", "zzznz", "zzzzs"], vec![(5, 5), (4, 4), (3, 3), (2, 2), (1, 1)]),
            TestCase(10, 10, vec!["kseeusenuk", "usesenesnn", "kskekeeses", "nesnusnkkn", "snenuuenke", "kukknkeuss", "neunnennue", "sknuessuku", "nksneekknk", "neeeuknenk"], vec![(9, 3), (8, 3), (7, 3), (6, 3), (5, 3)]),
            TestCase(5, 95, vec!["usknnsksussueuksuknukunsukneessneneeeuenkneeuksessusuesunnukknsnknsukuskeeskkuenseenkknnkeneuks", "uuknenkensksknkkusnnunnennskeueunsskekuuuueeknnekkesenunnkeeneuknksenukuuesekkeeeeunsueununsuee", "nsnensnkssnuksuknseeenseensnessunkseunsesnknukuneknsskueseukeeeskkeskeeneknusknesesnekuueeuesss", "eeekknnssuuuseskenkunusssknsskssuukssuekeknkssssensunueeeeukenkskuksuuuueueesnskseskksuuuusuksu", "ukukueksesksssnnneekksnnnnkenussunenneueuneeknskuskusuusneneunnusseuuseunkuukssnkennseskusknueu"], vec![(1, 47), (2, 46), (3, 45), (4, 44), (5, 43)]),
            TestCase(5, 96, vec!["xuvadgecswqwluhzzrslmzbkwyaslguyahbempkuxmvmwapyryjcoeptvifwxjlfnnffnjhekfupytmvcvwtbzaqvsdbbhwu", "oyiqaekunsqkrxaumhqqyjwxtlnmzqhfcgqiseezekrscopbsucurufwhjgvkxtftfkjwvusxtgjzogciqzvjvnfpkearbeq", "jmcssqmfzhnhewqrqrfpgnbwjagmwsrhuvluummbaxfxgmmigeceibtpufmxxwxhextmhzazzejrggkwrgfnitlfhehjpnit", "cjohxxslboylstgknzzatnycudotemvwhtlghwshycavmejynpsqpenddrmefybzbdjlirfezmvzcrvaosxchhjqtzgwntkc", "bosnwbdbnqxdewtcqlrvqwytfubogsmpffeipjfbmdpieaekzzuiffcszjwsktlxzwoyqjgfciaynvsiqusonehtbxzoveaq"], vec![(2, 10), (2, 9), (2, 8), (2, 7), (2, 6)]),
            TestCase(5, 5, vec!["sunne", "unsuk", "uunse", "usnsu", "ekuns"], vec![(5, 5), (5, 4), (5, 3), (5, 2), (5, 1)]),
            TestCase(98, 5, vec!["ensku", "keukn", "seesn", "kkkns", "knunk", "skneu", "sseeu", "nsnnu", "enkun", "eneue", "nsese", "ukskk", "nnssu", "sekun", "ksuse", "susun", "uuusu", "nksuu", "essks", "uksse", "sessk", "nusnn", "nseen", "eeesn", "nuuen", "kuuuu", "nkknn", "eknuk", "uskss", "eenek", "uueuu", "nksss", "unkuu", "ukeus", "unnkk", "suses", "ueunk", "esknn", "kueuu", "unnnn", "sessu", "kukuu", "kssuk", "eknes", "sknus", "kekuu", "ssknk", "sseun", "eukne", "kknes", "ueeen", "sekus", "nsske", "nennn", "unksu", "eseku", "neunu", "enuke", "enkeu", "ekkuu", "suseu", "seunu", "nknks", "sseku", "ueuss", "susue", "snuke", "eeekn", "nnuee", "knnkk", "usnsk", "eukek", "nknss", "sssue", "essun", "ensss", "nksue", "suksn", "seneu", "nkeke", "ukuuu", "kkkkn", "kuenn", "skkks", "kuesn", "unsne", "ekssk", "knnnu", "kkekn", "knnuk", "nsusk", "unknk", "esnee", "skess", "useeu", "kkkkn", "uessu", "nknse"], vec![(67, 1), (67, 2), (67, 3), (67, 4), (67, 5)]),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
