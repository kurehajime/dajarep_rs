use std::{
    fs::{self},
    io::{self, stdout, Read, Write},
};

use clap::Parser;

fn main() {
    let arg: Args = Args::parse();
    if arg.interactive {
        interactive();
        return;
    }

    let text = read(&arg);
    let result = dajarep::dajarep::dajarep(&text);

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

fn read(arg: &Args) -> String {
    let mut text = String::new();
    let mut s: Vec<u8> = Vec::new();
    match &arg.path {
        Some(path) => {
            s = fs::read(path).unwrap();
        }
        None => {
            // パイプが来ないことも普通にある
            match io::stdin().read_to_end(&mut s) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
    }
    match &arg.encode {
        Some(encode) => {
            if encode.to_lowercase() == "shift_jis"
                || encode.to_lowercase() == "sjis"
                || encode.to_lowercase() == "shift-jis"
            {
                let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
                text = res.into_owned();
            }
        }
        None => {
            let (res, _, _) = encoding_rs::UTF_8.decode(&s);
            text = res.into_owned();
        }
    }
    text
}

fn interactive() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        if word.trim().len() == 0 {
            break;
        }
        let result = dajarep::dajarep::is_dajare(&word);
        match result {
            Some(hit) => {
                println!("-> {}", hit);
            }
            None => {}
        }
    }
}

#[derive(Debug, Parser)]
#[clap(name = "struct", author, about, version)]
struct Args {
    #[clap(short, long, help = "encode")]
    encode: Option<String>,
    #[clap(short, long, help = "interactive mode")]
    interactive: bool,
    #[clap(help = "interactive mode")]
    path: Option<String>,
}
