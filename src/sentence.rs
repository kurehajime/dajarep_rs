use crate::word::Word;
#[derive(Debug)]
pub struct Sentence {
    pub str: String,
    pub kana: String,
    pub yomi: String,
    pub words: Vec<Word>,
}
