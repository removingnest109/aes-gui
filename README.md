# aes-gui

A tool for encrypting and decrypting text securely with a graphical user interface.

## About

This tool allows an input text to be encrypted or decrypted using password based encryption. This is done by using PBKDF2 to derive a unique AES256 key from the master password for each encrypted message.

## Installation (Windows)
To install aes-gui on windows, just download the aes-gui.exe file from the [Releases](https://github.com/removingnest109/aes-gui/releases) page.

The program is a standalone .exe and can be placed anywhere on the system without issues.

## Installation (Linux)
Executables can be found on the [Releases](https://github.com/removingnest109/aes-gui/releases) page.

To install the executable binary on linux, download and extract the aes-gui_x86_64-linux.tar.gz.

This archive contains a binary file that can be placed anywhere in the system, but it is best to place it somewhere that is included in your $PATH.

Alternatively, if you have Cargo installed, you can also build aes-gui from source:

```bash
git clone https://github.com/removingnest109/aes-gui.git

cd aes-gui

cargo install --path .
```

If you build the program using "cargo install" on linux, it will place the executable in ~/.cargo/bin/
