use compress::zlib;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let stream = File::open(&Path::new("obj2")).unwrap();
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
