slint::slint! {
    
    import { LineEdit, GridBox, Button, TextEdit } from "std-widgets.slint";

    export global Logic {
	in-out property<string> input: "Input";
	in-out property<string> key: "Key";
	in-out property<string> output: "Output";
	callback encrypt(string, string) -> string;
	callback decrypt(string, string) -> string;
    }

    export component App inherits Window {

	in-out property<string> input <=> Logic.input;
	in-out property<string> key <=> Logic.key;
	in-out property<string> output <=> Logic.output;
	callback encrypt <=> Logic.encrypt;
	callback decrypt <=> Logic.decrypt;
	
	title: "Aes text encryption";
	width: 600px;
	height: 800px;
	
	GridBox {
	    spacing: 5px;
	    width: parent.width;
            height: parent.height;

	    Row {
		input := TextEdit {
		    text: root.input;
		    font-size: 12px;
		    wrap: no-wrap;
		    colspan: 2;
		    edited(text) => {
			root.input = text;
		    }
		}
	    }

	    Row {
		key := LineEdit {
		    text: root.key;
		    font-size: 12px;
		    height: 5%;
		    colspan: 2;
		    edited(text) => {
			root.key = text;
		    }
		}
	    }
	    
	    Row {
		output := TextEdit {
		    text: root.output;
		    font-size: 12px;
		    read-only: true;
		    wrap: no-wrap;
		    colspan: 2;
		}
	    }
	    
	    Row {
		Button {
		    text: "Encrypt";
		    height:5%;
		    clicked => {
			root.output = encrypt(root.key, root.input)
		    }
		}
		
		Button {
		    text: "Decrypt";
		    height:5%;
		    clicked => {
			root.output = decrypt(root.key, root.input)
		    }
		}
	    }
	    
	}
    }
}

mod pbkdf2_aes;
use pbkdf2_aes::{encrypt, decrypt};

fn main() {
    match App::new() {
        Ok(app) => {
            let logic = app.global::<Logic>();
	    
            logic.on_encrypt(|key, input| {
                match encrypt(&key, &input) {
                    Ok(encrypted_data) => {
                        slint::SharedString::from(encrypted_data)
                    },
                    Err(err) => {
                        eprintln!("Encryption error: {:?}", err);
                        slint::SharedString::new() // Return an empty string on error
                    }
                }
            });

            logic.on_decrypt(|key, input| {
                match decrypt(&key, &input) {
                    Ok(decrypted_data) => {
                        slint::SharedString::from(decrypted_data)
                    },
                    Err(err) => {
                        eprintln!("Decryption error: {:?}", err);
                        slint::SharedString::new() // Return an empty string on error
                    }
                }
            });

            let _ = app.run();
        }
        Err(err) => {
            eprintln!("Error creating app: {:?}", err);
        }
    }
}
