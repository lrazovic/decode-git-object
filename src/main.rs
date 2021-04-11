use clap::{App, Arg};
use compress::zlib;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let matches = App::new("Decode Git Object")
        .version("0.1.0")
        .author("Leonardo Razovic <lrazovic@gmail.com>")
        .about("Teaches argument parsing")
        .arg(
            Arg::with_name("file")
                .required(true)
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The object file to decode"),
        )
        .get_matches();
    let filename = matches.value_of("file").unwrap();
    let file: File;
    let stream = File::open(&Path::new(filename));
    match stream {
        Ok(data) => file = data,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    }
    let mut decompressed = Vec::new();
    match zlib::Decoder::new(file).read_to_end(&mut decompressed) {
        Ok(_) => match String::from_utf8(decompressed) {
            Ok(str) => {
                println!("{}", str);
                std::process::exit(0)
            }
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1)
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
