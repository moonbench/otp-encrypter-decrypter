use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use clap::{Arg, App, AppSettings, SubCommand};

fn encrypt(input_filename: &str, pad_filename: &str) {
    let output_filename = format!("{}.encrypted", input_filename).to_string();
    println!("Encrypting {}...", &input_filename);
    convert(input_filename, pad_filename, &output_filename);
}

fn decrypt(input_filename: &str, pad_filename: &str) {
    let mut output_filename: Vec<&str> = input_filename.split('.').collect();
    output_filename.truncate(output_filename.len()-1);
    println!("Decrypting {}...", &input_filename);
    convert(input_filename, pad_filename, &output_filename.join("."));
}

fn convert(input_filename: &str, pad_filename: &str, output_filename: &str) {
    let input_path = Path::new(&input_filename);
    let input_file = File::open(&input_path).expect("[ ERROR ] Failed to open plaintext file");
    let pad_path = Path::new(pad_filename);
    let pad_file = File::open(&pad_path).expect("[ ERROR ] Failed to open one-time pad file");
    let mut output_file = File::create(&output_filename).expect("[ ERROR ] Failed to create output file");

    let mut pad_buffer = pad_file.bytes();
    let input_buffer = input_file.bytes();
    let mut bytes: Vec<u8> = Vec::new();
    for byte in input_buffer {
        let pad_byte = pad_buffer.next().expect("[ ERROR ] One-time pad buffer is empty");
        let pad_byte = pad_byte.expect("[ ERROR ] Unable to read from one-time pad file");
        let byte = byte.expect("[ ERROR ] Unable to read from input file");
        bytes.push(byte ^ pad_byte);
    }
    output_file.write_all(&bytes).expect("[ ERROR ] Failed to write to output file");
    println!("Created {}", &output_filename);
    println!("Done.");
}

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("encrypt")
                    .about("Encrypts a file")
                    .arg(Arg::with_name("input")
                        .long("input")
                        .short("i")
                        .help("The file to be encrypted")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("pad")
                        .long("pad")
                        .short("p")
                        .help("The one-time pad")
                        .takes_value(true)
                        .required(true)))
                .subcommand(SubCommand::with_name("decrypt")
                    .about("Decrypts a file")
                    .arg(Arg::with_name("input")
                        .long("input")
                        .short("i")
                        .help("The file to be decrypted")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("pad")
                        .long("pad")
                        .short("p")
                        .help("The one-time pad")
                        .takes_value(true)
                        .required(true)))
                .get_matches();

    match args.subcommand() {
        ("encrypt", sub_args) => {
            let sub_args = sub_args.unwrap();
            encrypt(
                sub_args.value_of("input").unwrap(),
                sub_args.value_of("pad").unwrap());
        },
        ("decrypt", sub_args) => {
            let sub_args = sub_args.unwrap();
            decrypt(
                sub_args.value_of("input").unwrap(),
                sub_args.value_of("pad").unwrap());
        },
        _ => unreachable!()
    }
}
