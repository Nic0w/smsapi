use std::io;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    pub user: String,

    #[clap(short='k', long)]
    pub api_key: String,

    pub message: Option<String>
}

fn main() {

    let args = Args::parse();

    let api = smsapi::authenticate(&args.user, &args.api_key);

    let message = args.message.unwrap_or_else(|| {

        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .expect("reading from standard input");

        line
    });

    let result = api.send(&message);

    println!("{:?}", result);
}