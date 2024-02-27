use std::io;
use bip39::{Mnemonic, MnemonicType, Language};
// use solana_sdk::signature::{keypair_from_seed_phrase_and_passphrase, write_keypair_file};

#[allow(warnings)]
fn main() {
    let mut input = String::new();
        println!("Type the length: 12 or 24");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim() == "12" || input.trim() == "24" {
                let mnemonic = Mnemonic::new(MnemonicType::for_word_count(input.trim().parse().unwrap()).unwrap(), Language::English);
                println!("Mnemonic: {:?}", mnemonic.phrase());
            } else {
                println!("Invalid length");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}