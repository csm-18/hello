
const VERSION:&str = "1.0.0";
fn main() {
    //cli args
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

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
            dbg!("compiling!");
        }
    }else if args.len() == 0 {
        println!("hello {VERSION}");
        println!("for more info:");
        println!(" hello help");
    }else {
        println!("Too many args!");
    }
}
