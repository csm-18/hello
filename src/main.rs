mod compiler;

use std::fs;
use std::process::exit;
use crate::compiler::compile;

//hello version
const VERSION:&str = "1.0.0";
fn main() {
    //cli args
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    //process cli args
    if args.len() == 1 {
        if args[0] == "version" {
            println!("hello {VERSION}");
        }else if args[0] == "help" {
            println!("hello commands:");
            println!(" 1) hello <no args>    --> prints about hello");
            println!(" 2) hello version      --> prints hello version");
            println!(" 3) hello help         --> prints this commands list");
            println!(" 4) hello <filename.h> --> compiles hello source file into x86 binary.");
        }else {
            //source file name
            let filename = &args[0];

            if !(filename.len() >= 3){
                println!("'{filename}' is not a valid hello-source-file!");
                exit(1);
            }
            else if &filename[filename.len()-2..] != ".h" {
                println!("'{filename}' is not a valid hello-source-file!");
                exit(1);
            }

            //read code from source file
            let code = match fs::read_to_string(&filename) {
                Ok(code) => code,
                Err(_error) => {
                  println!("Unable to open '{filename}'!");
                  exit(1);
                },
            };

            //compile to assembly
            let output_asm = compile(&code);
            dbg!(output_asm);

        }
    }else if args.len() == 0 {
        println!("hello {VERSION}");
        println!("for more info:");
        println!(" hello help");
    }else {
        println!("Too many args!");
    }
}
