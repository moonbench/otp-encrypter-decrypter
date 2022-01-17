# OTPEncrypt
Encrypt and/or decrypt any type of file using one-time pads.

## Purpose
Uses a user-provided one-time pad to encrypt or decrypt an input file's contents, byte by byte. It can be used with any type of file.

Pads for this program can be generated using a tool such as https://github.com/moonbench/otpgen

## Installation
* Clone the repository
* Run `cargo build --release`

## Usage
```
USAGE:
    otpencrypt <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decrypt    Decrypts a file
    encrypt    Encrypts a file
    help       Prints this message or the help of the given subcommand(s)
```

### Encryption
```
USAGE:
    otpencrypt encrypt --input <input> --pad <pad>

OPTIONS:
    -i, --input <input>    The file to be encrypted
    -p, --pad <pad>        The one-time pad
```

### Decryption
```
USAGE:
    otpencrypt decrypt --input <input> --pad <pad>

OPTIONS:
    -i, --input <input>    The file to be decrypted
    -p, --pad <pad>        The one-time pad
```

## Examples
### Encrypting a file
To encrypt the file `document.txt` using the pad `onetimepad`:

```
./target/release/otpencrypt encrypt --input document.txt --pad onetimepad
```

This will produce an encrypted file `document.txt.encrypted`.

### Decrypting a file
To decrypt the file `document.txt.encrypted` using the pad `onetimepad`:

```
./target/release/otpencrypt decrypt --input document.txt.encrypted --pad onetimepad
```

This will produce the original `document.txt` file if the same pad is used for both operations.
