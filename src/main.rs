use std::path::PathBuf;

use clap::Parser;

fn main() {
    let text = r#"
人民の人民による人民のための政治
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
"#;
    let result = dajarep::dajarep(text);

    match result {
        Ok(words) => {
            for word in words {
                println!("{}", word);
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

#[derive(Debug, Parser)]
#[clap(name = "struct", author, about, version)]
struct Args {
    #[clap(short, long, help = "encode")]
    encode: Option<String>,
}
