extern crate syntax_checker;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use syntax_checker::check;

fn main() {
    let myfile = File::open("nene.txt").expect("Fail to open file");
    let mut reader = BufReader::new(myfile);
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .expect("Fail to read from file");
    let msg = String::from("Hello this is a test {};");
    let mut chk = check(msg);
    println!("{}", chk);
    println!("{}", content);
    chk = check(content);
    println!("{}", chk);
}
