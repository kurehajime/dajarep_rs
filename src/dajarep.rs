use crate::sentence::Sentence;
use crate::word::Word;
use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig},
    DictionaryKind,
};
use regex::Regex;
use std::fmt::Error;

pub fn dajarep(text: &str) -> Result<Vec<String>, Error> {
    let sentences = get_sentences(text).unwrap();
    let mut dajares = vec![];

    for sentence in sentences {
        match is_dajare_by_sentence(&sentence) {
            Some(_) => {
                dajares.push(sentence.str);
            }
            None => {}
        };
    }
    Ok(dajares)
}

pub fn is_dajare(word: &str) -> Option<String> {
    let sentences = get_sentences(word).unwrap();
    if sentences.len() > 0 {
        is_dajare_by_sentence(&sentences[0])
    } else {
        None
    }
}

fn is_dajare_by_sentence(sentence: &Sentence) -> Option<String> {
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

pub fn get_sentences(text: &str) -> Result<Vec<Sentence>, Error> {
    let mut sentences: Vec<Sentence> = Vec::new();
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };
    let config = TokenizerConfig {
        dictionary,
        mode: Mode::Normal,
        user_dictionary: None,
    };
    let tokenizer = Tokenizer::with_config(config).unwrap();

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
        let tokens = tokenizer.tokenize_with_details(s).unwrap();
        let mut words: Vec<Word> = Vec::new();
        let mut kana = String::new();
        let mut yomi = String::new();
        for t in tokens {
            let details = t.details.unwrap();
            if details.len() < 8 {
                continue;
            }

            let w = Word {
                str: t.text.to_string(),
                kana: details[7].clone(),
                wtype: details[0].clone(),
            };
            kana = kana + &details[7].clone();
            yomi = yomi + &details[8].clone();
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

fn replace_regex(text: &str, from: &str, to: &str) -> String {
    Regex::new(from).unwrap().replace_all(&text, to).to_string()
}

fn count_regex(text: &str, target: &str) -> i32 {
    let re = Regex::new(target).unwrap();
    let caps = re.find_iter(text).count();
    caps.try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dajarep() {
        let input = r#"人民の人民による人民のための政治
アルミ缶の上にあるミカン
トンネルを抜けるとそこは雪国であった
智代子のチョコ
布団が吹っ飛んだ
我輩は猫である
猫が寝転んだ
その意見にはついていけん
靴を靴箱に入れる
傘を貸さない
イカは如何なものか
親譲りの無鉄砲で子供の時から損ばかりしている
マイケル・ジョーダンが冗談を言った
知事が縮む
鶏には取り憑かない
破壊についての和解"#;
        let answer = r#"アルミ缶の上にあるミカン
智代子のチョコ
布団が吹っ飛んだ
猫が寝転んだ
その意見にはついていけん
傘を貸さない
イカは如何なものか
マイケル・ジョーダンが冗談を言った
知事が縮む
鶏には取り憑かない"#
            .split("\n");

        let result = dajarep(input).unwrap();
        assert_eq!(answer.clone().count(), result.len());
        answer.zip(result.iter()).for_each(|(a, r)| {
            assert_eq!(a, r);
        });
    }
}
