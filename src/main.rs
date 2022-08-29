use clap::Parser;
use rc4::Rc4;
use std::fs::File;
use std::io::prelude::{Read, Seek, Write};
use rand::{rngs::OsRng, RngCore};
mod encrypt;
use encrypt::*;
#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, required = true, value_name = "FILE_NAME")]
    file: String,
    #[clap(
        short,
        long,
        required = true,
        value_name = "HEX_BYTES",
        min_values = 1,
        max_values = 256
    )]
    key: String,
}

fn main() {
    let args = Args::parse();
    let mut contents = Vec::new();
    let mut file = File::options().read(true).write(true).open(&args.file).unwrap();
    file.read_to_end(&mut contents).unwrap();

    let mut small_file_nonce = [0u8; 24];
    OsRng.fill_bytes(&mut small_file_nonce);
    


    encrypt_small_file(args.file.as_str(),"file.enc",args.key.as_str().as_bytes(), &small_file_nonce);
   
     
}