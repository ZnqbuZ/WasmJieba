#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cut() {
    let test_sentences = [
        "数字化转型是否促进了企业内共同富裕?——来自中国A股上市公司的证据",
        "改革歷程",
        "我们中出了一个叛徒",
        "我来到北京清华大学",
        "他来到了网易杭研大厦",
        "小明硕士毕业于中国科学院计算所，后在日本京都大学深造",
        "实变函数论与泛函分析",
        "實變函數論與泛函分析",
        "梅竹錦標對抗賽",
        "小明畢業於國立交通大學資訊科學與工程研究所",
        "新竹的交通大學要在2021年2月1日與台北的陽明大學合併",
    ];
    // wasmjieba::addWord("歷程", Some(1048576), Some("n".to_string()));
    for sentence in test_sentences {
        let result = wasmjieba::cut(sentence, true);
        println!("{:?}", result);
    }
}
