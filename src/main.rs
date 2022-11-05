use std::{
    fs::File,
    io::{self, stdout, Read, Write},
};

use clap::Parser;

fn main() {
    let arg: Args = Args::parse();
    let mut text = String::new();

    if arg.interactive {
        interactive();
        return;
    }

    match &arg.path {
        Some(path) => {
            text = read_file(path);
        }
        None => {
            text = read_pipe();
        }
    }

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
    println!("{:?}", arg);
}

fn read_file(file: &str) -> String {
    let mut file = File::open(file).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("can't read file.");
    contents
}

fn read_pipe() -> String {
    let mut contents = String::new();
    io::stdin()
        .read_to_string(&mut contents)
        .expect("pipe error");
    println!("----{}", contents);
    contents
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
