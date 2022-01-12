# OTP-encrypter-decrypter
A tool to encrypt and decrypt files using one-time pads.

Pads for this program can be generated using https://github.com/moonbench/otp-gen

## Purpose
Uses a user-specified one-time pad to permute an input file, byte by byte, to either encrypt or decrypt its contents.

It can be used with any type of file.

## Installation
* Clone the repository
* Run `cargo build --release`

## Usage
```
Usage: otpencrypt <command> <input file> <one-time pad>

The available commands are 'encrypt' and 'decrypt'.
```

### Encrypting a file
To encrypt the file `plaintext.txt` using the pad `onetimepad`

```./target/release/otpencrypt encrypt plaintext.txt onetimepad```

This will produce the encrypted file `plaintext.txt.encrypted`

### Decrypting a file
To decrypt the file `plaintext.txt.encrypted` using the same pad `onetimepad`

```./target/release/otpencrypt decrypt plaintext.txt.encrypted onetimepad```

This will produce the original `plaintext.txt` file if the same pad is used for both operations.
