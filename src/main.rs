use std::{fs::File, io::Read};

use clap::Parser;

fn main() {
    let arg: Args = Args::parse();
    let mut text = String::new();

    match &arg.path {
        Some(path) => {
            text = read_file(path);
        }
        None => {}
    }

    let result = dajarep::dajarep(&text);

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
    println!("{:?}", arg);
}

fn read_file(file: &str) -> String {
    let mut file = File::open(file).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("can't read file.");
    contents
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
