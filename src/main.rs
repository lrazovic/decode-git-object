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
    let stream = File::open(&Path::new(filename)).unwrap();
    let mut decompressed = Vec::new();
    match zlib::Decoder::new(stream).read_to_end(&mut decompressed) {
        Ok(_) => {
            let obj = String::from_utf8(decompressed);
            match obj {
                Ok(str) => {
                    println!("{}", str)
                }
                Err(_) => {
                    eprintln!("Eror in UTF-8 Decoding")
                }
            }
        }
        Err(_) => {
            eprintln!("Error in Zlib Decompressing")
        }
    }
}
