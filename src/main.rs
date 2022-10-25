fn main() {
    let text = "布団が吹っ飛んだ";
    let sentences = dajarep::getSentences(text).unwrap();
    let result = dajarep::isDajare(&sentences[0]);

    match result {
        Some(word) => {
            println!("「{}」はダジャレです。({})", text, word);
        }
        None => {}
    }
}
