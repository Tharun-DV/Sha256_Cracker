#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use sha2::digest::crypto_common::ParBlocksSizeUser;
use sha2::{Sha256,Digest};
use std::process::exit;


fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        println!("Invalid Ammount of Arguments");
        println!("`./<package name>` `hash`");
        exit(0);
    }

    let wanted_hash :&String = &args[1];
    let password_file = "src/rockyou.txt";

    let mut attempt:u32 = 1;
    println!("Attempting to crack the hack : {wanted_hash}!\n");
    let password_list = File::open(password_file).expect("cannot open password wordlist");
    let reader = BufReader::new(password_list);

    for line in reader.lines(){
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}",Sha256::digest(&password));
        println!("[{}] {} == {}",attempt,std::str::from_utf8(&password).unwrap(),password_hash);

        if &password_hash == wanted_hash{
            println!("Password hash found after {} attempts! {} hashes to {}!",attempt,std::str::from_utf8(&password).unwrap(),password_hash);
            exit(0);
        }  
        attempt +=1;
    }
        println!("Password hash not found!");
}

