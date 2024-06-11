# aes-gui

A tool for encrypting and decrypting text securely with a graphical user interface.

Uses Rust for the encryption logic, and slint for ui framework

Wasm bindings are already setup, see "[Compile for web](compile-for-web)" below for more info

## About

This tool allows an input text to be encrypted or decrypted using password based encryption. This is done by using PBKDF2 to derive a unique AES256 key from the master password for each encrypted message.

## Installation (Windows)
To install aes-gui on windows, just download the aes-gui.exe file from the [Releases](https://github.com/removingnest109/aes-gui/releases/latest) page.

The program is a standalone .exe and can be placed anywhere on the system without issues.

## Installation (Linux)
Executables can be found on the [Releases](https://github.com/removingnest109/aes-gui/releases/latest) page.

To install the executable binary on linux, download and extract the aes-gui_x86_64-linux.tar.gz.

This archive contains a binary file that can be placed anywhere in the system, but it is best to place it somewhere that is included in your $PATH.

Alternatively, if you have Cargo installed, you can also build aes-gui from source:

```bash
git clone https://github.com/removingnest109/aes-gui.git

cd aes-gui

cargo install --path .
```

If you build the program using "cargo install" on linux, it will place the executable in ~/.cargo/bin/

## Compile for web
aes-gui can be compiled into a frontend only webapp and hosted on github pages or similar services

You will need to have wasm-pack installed to compile for web:
```bash
cargo install wasm-pack
```

Then the target can be built. 
```bash
git clone https://github.com/removingnest109/aes-gui.git

cd aes-gui

wasm-pack build --target web
```

This builds to ./pkg/

To host this web app, the index.html file and pkg folder must be in the same directory, then the directory can be added to github pages or hosted using a basic http server

```bash
python3 -m http.server 8000
```
