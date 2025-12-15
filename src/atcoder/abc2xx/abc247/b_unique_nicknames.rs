// https://atcoder.jp/contests/abc247/tasks/abc247_b

fn run(n: usize, a: Vec<[&str; 2]>) -> &'static str {
    for i in 0..n {
        let mut dist = 0;

        for j in 0..=1 {
            for k in 0..n {
                if i == k {
                    continue;
                }

                if a[k].contains(&a[i][j]) {
                    dist += 1;
                }
            }
        }

        if dist == 2 {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<[&'static str; 2]>, &'static str);

    #[test]
    fn abc247_b() {
        let tests = [
            TestCase(3, vec![["tanaka", "taro"], ["tanaka", "jiro"], ["suzuki", "hanako"]], "Yes"),
            TestCase(3, vec![["aaa", "bbb"], ["xxx", "aaa"] ,["bbb", "yyy"]], "No"),
            TestCase(2, vec![["tanaka", "taro"], ["tanaka", "taro"]], "No"),
            TestCase(3, vec![["takahashi", "chokudai"], ["aoki", "kensho"], ["snu", "ke"]], "Yes"),
            TestCase(100, vec![["avmlajlxrf", "zttmjemxex"], ["vauvmzqhzs", "uzgvclqnem"], ["lcljwpbfsg", "xnbsoouwsn"], ["zifcqxivtq", "gjcupzgkyz"], ["cinpwuotzy", "yxojlpipjn"], ["tdnemryudq", "skxibydrct"], ["tdjdtgtipl", "eurwjwxvyv"], ["abzttwzxhb", "amycbijxta"], ["vwzjmmlaew", "yxojlpipjn"],["xteucdfimo", "syiwqxsyvy"],["vowbgfxsqv", "sjcegwybmg"],["urdgjzthal", "cckapzefmh"], ["jlqemwztfg", "yzlusdaezt"], ["ejwwcutzsa", "atxkasugou"], ["fvvvsecqvg", "csloaexqar"], ["xdudibkgmt", "dmzuyjjkfu"], ["jnmfxnucto", "anlkpmegdk"], ["hlwnwahibo", "wdwgecsbqe"], ["xoeqypczez", "txgzzkdeco"], ["vrwhijewvl", "wytsxzogzb"], ["flocshjexi", "vrounuangs"], ["giiikhuvlw", "eapudjjlxz"], ["asvwjrhpck", "oyyrexwiri"], ["cqsqohapin", "ddpdojipiq"], ["hgtmfsuedp", "tkjqmgetkl"],["emrjmdsahn", "urmlzavbyc"], ["odultqflzd", "obnwbmbrbv"], ["jhfxadevqt", "cinpwuotzy"],["dyvarcvxms", "tduoqcljzw"], ["pcumzuyzgs", "xbfsoxvtlx"],["slzgvyptkq", "begswrioai"], ["sikliwzvqd", "ivkzfnjnqs"], ["fxfomljyeo", "rohrqgsbrr"], ["mwtptktucb", "gabwlmmezw"], ["swydbovjqx", "ybfbjxmlkw"], ["qhpbuccyhu", "dsaxxwfler"], ["guqxzzgrlr", "ftimlzorzv"], ["bqklofbwzb", "pyemgoqbqp"], ["xnzsfrgfqd", "fctpafbged"], ["safprjkxav", "iducfibsms"],["bgaczxkjjo", "qdubpubmqy"], ["gjfgwuapdu", "mxngjzuyzs"],["gfmrbrczyv", "wkybysnbzh"], ["cpqnmbijhc", "tisbxwevsl"], ["yyepttinlp", "qxdlcyaryl"], ["rtfvtlsaed", "gxnmgfdlaz"], ["cztjbttosy", "xvgevjycvc"], ["niqyddonks", "pcntudhjex"], ["eprhqytehu", "gxoechnwzq"], ["uewhzvtawt", "hdkzmpoxdd"], ["qrtpqcgzid", "jmdnedpvde"], ["cbobbqkhvo", "fecwfsshrs"], ["lxlwxjdune", "nbfsjuwwfl"], ["nfolbzlgjz", "ayiiidkxpv"], ["secemmlcyo", "binjogcrin"], ["evzuvmvvxi", "wsxrlspmwe"],["edufzrlfzz", "uhzqiupfoc"], ["cjehxnsfew", "lfxcujwugl"],["vfvxpvhkzp", "zwhyamzuqc"], ["bmhndupair", "jmijqprnqv"],["rrxvdkpxxe", "comgvwojrd"], ["shhltqafiy", "axpqnlimij"],["qwjlqpuqyr", "rwrkgculnr"], ["isiadtwyew", "gmgrjchxeq"],["asydjlrgnu", "rsqxsdaeke"], ["byeogugzvd", "oimjblsaqp"],["etfzxishqr", "zicggawrny"], ["vrkvpixskp", "pootisasgb"],["jyquzkympl", "kpgrarjurw"], ["irokbsgmzn", "axnmijtbuo"],["jcicrhohws", "glmxshpofv"], ["mlcltqliqd", "edgalwqyrz"],["qabdaqkuvk", "gpwzxamavm"], ["rjuybctves", "qoltuirrbt"],["ceehfrnnlt", "wzqpfbegeb"], ["bygsepvzvs", "wivbkxcwwr"],["bztgqkaszz", "kbddlinrdl"], ["bgcwkfhjfd", "anvaxdphvm"],["hmkrfrrykj", "tposlkgyfu"], ["cvpzdchrcf", "jrkecgpdod"],["ziyqvltwjr", "jjywyobhow"], ["mspuosmxsw", "plkwluwjpg"],["lnxxowqjmu", "eggxviqrxd"], ["dgwlamwtdf", "svrlcuukan"], ["edrefbtrce", "khqwssekyx"], ["uhcvrasuxf", "vtvrbfeswq"], ["jupyipafzd", "ufewjurxbj"], ["bltjuxbtkv", "zmfhstxyzx"], ["zgcxjwddcs", "xafyqtugis"], ["glbrpmckrt", "xqxemkieyn"], ["epbfldpccl", "idkkwdfvrc"], ["rlcsstqdlx", "rvdzydkjnp"], ["rhknmgmqdq", "xyumddhczm"], ["ignokmfecc", "tizaaxfkgh"], ["zlinrzhweq", "zzdpcqbjvc"], ["emehoqjlcq", "gmvctsblgw"], ["yargqiezlu", "oyifpuekox"], ["ntveljeitx", "coyexswfvj"], ["nbejnmvkdn", "amgirkcihn"], ["lxlaawoatn", "qjofmdhvwt}"]], "No"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
