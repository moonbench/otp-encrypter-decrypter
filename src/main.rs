use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::num::Wrapping;

fn encrypt(input_filename: &str, pad_filename: &str) {
    let output_filename = format!("{}.encrypted", input_filename).to_string();
    convert(input_filename, pad_filename, &output_filename, true);
}

fn decrypt(input_filename: &str, pad_filename: &str) {
    let mut output_filename: Vec<&str> = input_filename.split('.').collect();
    output_filename.truncate(output_filename.len()-1);
    convert(input_filename, pad_filename, &output_filename.join("."), false);
}

fn convert(input_filename: &str, pad_filename: &str, output_filename: &str, is_encrypting: bool) {
    println!("[ INFO ] Input files: {}", &input_filename);
    println!("[ INFO ] One-time pad: {}", &pad_filename);
    println!("[ INFO ] Output file: {}", &output_filename);

    let input_path = Path::new(&input_filename);
    let input_file = File::open(&input_path).expect("[ Error ] Failed to open plaintext file");
    let pad_path = Path::new(pad_filename);
    let pad_file = File::open(&pad_path).expect("[ Error ] Failed to open one-time pad file");
    let mut output_file = File::create(&output_filename).expect("[ ERROR ] Failed to create output file");

    let mut pad_buffer = pad_file.bytes();
    let input_buffer = input_file.bytes();
    let mut bytes: Vec<u8> = Vec::new();
    for byte in input_buffer {
        let pad_byte = pad_buffer.next().expect("[ ERROR ] One-time pad buffer is empty");
        let pad_byte = pad_byte.expect("[ ERROR ] Unable to read from one-time pad file");
        let byte = byte.expect("[ ERROR ] Unable to read from input file");
        if is_encrypting {
            bytes.push((Wrapping(byte) + Wrapping(pad_byte)).0);
        } else {
            bytes.push((Wrapping(byte) - Wrapping(pad_byte)).0);
        }
    }
    output_file.write_all(&bytes).expect("[ ERROR ] Failed to write to output file");
}

fn title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(&format!(" (v{}) ", env!("CARGO_PKG_VERSION")).to_string());
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn usage() {
    println!("{}", title());
    println!("Usage: {} <command> <input file> <one-time pad>", env!("CARGO_PKG_NAME"));
    println!("\nThe available commands are 'encrypt' and 'decrypt'.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        4 => {
            match &args[1][..] {
                "encrypt" => encrypt(&args[2], &args[3]),
                "decrypt" => decrypt(&args[2], &args[3]),
                _ => usage()
            }
        },
        _ => usage()
    }
}
