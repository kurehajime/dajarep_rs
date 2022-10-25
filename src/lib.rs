mod sentence;
mod word;

use lindera::tokenizer::Tokenizer;
use regex::Regex;
use sentence::Sentence;
use std::fmt::Error;
use word::Word;

pub fn is_dajare(sentence: &Sentence) -> Option<String> {
    for word in &sentence.words {
        if word.wtype == "名詞" && word.kana.len() > 1 {
            let w_str = &word.str;
            let w_kana = &word.kana;
            let str_count = count_regex(&sentence.str, w_str);
            let kana_count_1 = count_regex(&sentence.kana, &fix_word(w_kana));
            let kana_count_2 = count_regex(&fix_sentence(&sentence.kana), w_kana);
            let kana_count_3 = count_regex(&sentence.yomi, w_kana);
            let kana_count_4 = count_regex(&fix_sentence(&sentence.yomi), w_kana);
            let kana_array = vec![kana_count_1, kana_count_2, kana_count_3, kana_count_4];
            let max_kana_count = kana_array.iter().max().unwrap();

            if str_count < *max_kana_count {
                return Some(word.kana.to_string());
            }
        }
    }

    None
}

fn count_regex(text: &str, target: &str) -> i32 {
    let re = Regex::new(target).unwrap();
    let caps = re.find_iter(text).count();
    caps.try_into().unwrap()
}

//置き換え可能な文字を考慮した正規表現を返す。
fn fix_word(text: &str) -> String {
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
    let text = replace_regex(&text, "([アカサタナハマヤラワャ])ー", "$1[アァ]?");
    let text = replace_regex(&text, "([イキシチニヒミリ])ー", "$1[イィ]?");
    let text = replace_regex(&text, "([ウクスツヌフムユルュ])ー", "$1[ウゥ]?");
    let text = replace_regex(&text, "([エケセテネへメレ])ー", "$1[エェ]?");
    let text = replace_regex(&text, "([オコソトノホモヨロヲョ])ー", "$1[ウゥオォ]?");
    let text = text.replace("ャ", "[ヤャ]");
    let text = text.replace("ュ", "[ユュ]");
    let text = text.replace("ョ", "[ヨョ]");
    let text = text.replace("ー", "[ー]?");
    text
}
fn replace_regex(text: &str, from: &str, to: &str) -> String {
    Regex::new(from).unwrap().replace_all(&text, to).to_string()
}
//本文から省略可能文字を消したパターンを返す。
fn fix_sentence(text: &str) -> String {
    let text = text.replace("ッ", "");
    let text = text.replace("ー", "");
    let text = text.replace("、", "");
    let text = text.replace(",", "");
    let text = text.replace("　", "");
    let text = text.replace("　", "");
    let text = text.replace(" ", "");
    text
}

pub fn get_sentences(text: &str) -> Result<Vec<Sentence>, Error> {
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
