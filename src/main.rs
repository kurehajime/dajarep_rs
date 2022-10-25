fn main() {
    let text = "布団が吹っ飛んだ";
    let sentences = dajarep::get_sentences(text).unwrap();
    let result = dajarep::is_dajare(&sentences[0]);

    match result {
        Some(word) => {
            println!("「{}」はダジャレです。({})", text, word);
        }
        None => {}
    }
}
