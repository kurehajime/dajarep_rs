use regex::Regex;
use std::fmt::Error;

use lindera::tokenizer::Tokenizer;
use sentence::Sentence;
use word::Word;

mod sentence;
mod word;

pub fn isDajare(sentence: &Sentence) -> Option<String> {
    for word in &sentence.words {
        if word.wtype == "名詞" && word.kana.len() > 1 {
            let rStr = &word.str;
            let rKana = &word.kana;
            let hitStr = countx(&sentence.str, rStr);
            let hitKana1 = countx(&sentence.kana, &fixWord(rKana));
            let hitKana2 = countx(&fixSentence(&sentence.kana), rKana);
            let hitKana3 = countx(&sentence.yomi, rKana);
            let hitKana4 = countx(&fixSentence(&sentence.yomi), rKana);
            let kanaArray = vec![hitKana1, hitKana2, hitKana3, hitKana4];
            let maxKanaCount = kanaArray.iter().max().unwrap();

            if hitStr < *maxKanaCount {
                return Some(word.kana.to_string());
            }
        }
    }

    None
}

fn countx(text: &str, target: &str) -> i32 {
    let re = Regex::new(target).unwrap();
    let caps = re.find_iter(text).count();
    caps.try_into().unwrap()
}

//置き換え可能な文字を考慮した正規表現を返す。
fn fixWord(text: &str) -> String {
    let text = text.replace("ッ", "[ツッ]?");
    let text = text.replace("ァ", "[アァ]?");
    let text = text.replace("ィ", "[イィ]?");
    let text = text.replace("ゥ", "[ウゥ]?");
    let text = text.replace("ェ", "[エェ]?");
    let text = text.replace("ォ", "[オォ]?");
    let text = text.replace("ズ", "[ズヅ]");
    let text = text.replace("ヅ", "[ズヅ]");
    let text = text.replace("ヂ", "[ジヂ]");
    let text = text.replace("ジ", "[ジヂ]");
    let text = replacex(text, "([アカサタナハマヤラワャ])ー", "$1[アァ]?");
    let text = replacex(text, "([イキシチニヒミリ])ー", "$1[イィ]?");
    let text = replacex(text, "([ウクスツヌフムユルュ])ー", "$1[ウゥ]?");
    let text = replacex(text, "([エケセテネへメレ])ー", "$1[エェ]?");
    let text = replacex(text, "([オコソトノホモヨロヲョ])ー", "$1[ウゥオォ]?");
    let text = text.replace("ャ", "[ヤャ]");
    let text = text.replace("ュ", "[ユュ]");
    let text = text.replace("ョ", "[ヨョ]");
    let text = text.replace("ー", "[ー]?");
    text
}
fn replacex(text: String, from: &str, to: &str) -> String {
    Regex::new(from).unwrap().replace_all(&text, to).to_string()
}
//本文から省略可能文字を消したパターンを返す。
fn fixSentence(text: &str) -> String {
    let text = text.replace("ッ", "");
    let text = text.replace("ー", "");
    let text = text.replace("、", "");
    let text = text.replace(",", "");
    let text = text.replace("　", "");
    let text = text.replace("　", "");
    let text = text.replace(" ", "");
    text
}

pub fn getSentences(text: &str) -> Result<Vec<Sentence>, Error> {
    let mut sentences: Vec<Sentence> = Vec::new();
    let tokenizer = Tokenizer::new().unwrap();

    // 終了系の文字列を改行に置き換え
    let text = text.replace("。", "\n");
    let text = text.replace(".", "\n");
    let text = text.replace("?", "?\n");
    let text = text.replace("!", "!\n");
    let text = text.replace("？", "？\n");
    let text = text.replace("！", "！\n");

    // 配列化
    let senstr = text.split("\n");

    // Sentence作成
    for s in senstr {
        let tokens = tokenizer.tokenize(s).unwrap();
        let mut words: Vec<Word> = Vec::new();
        let mut kana = String::new();
        let mut yomi = String::new();
        for t in tokens {
            let w = Word {
                str: t.text.to_string(),
                kana: tokenizer.word_detail(t.word_id).unwrap()[7].clone(),
                wtype: tokenizer.word_detail(t.word_id).unwrap()[0].clone(),
            };
            kana = kana + &tokenizer.word_detail(t.word_id).unwrap()[7].clone();
            yomi = yomi + &tokenizer.word_detail(t.word_id).unwrap()[8].clone();
            words.push(w);
        }
        let sentence = Sentence {
            str: s.to_string(),
            kana: kana,
            yomi: yomi,
            words,
        };
        sentences.push(sentence);
    }
    Ok(sentences)
}
