use lindera::tokenizer::{DictionaryConfig, DictionaryKind, Tokenizer, TokenizerConfig};
use lindera::LinderaResult;

fn main() -> LinderaResult<()> {
    // create tokenizer
    let dictionary = DictionaryConfig {
        kind: DictionaryKind::IPADIC,
        path: None,
    };
    let config = TokenizerConfig {
        mode: lindera::mode::Mode::Normal,
        dictionary: dictionary,
        user_dictionary: None,
    };
    let tokenizer = Tokenizer::with_config(config)?;

    // tokenize the text
    let tokens = tokenizer.tokenize("東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です")?;

    // output the tokens
    for token in tokens {
        println!("{},{:?}", token.text, tokenizer.word_detail(token.word_id));
    }

    Ok(())
}
