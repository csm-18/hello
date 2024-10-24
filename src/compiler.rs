//compiler.rs : contains actual compiler code

//converts hello code into assembly and returns it
pub fn compile(src:&str) -> String {
    //output(assembly code)
    let output: String = "output!".to_owned();

    let tokens = lexer(src);

    output
}

//creates tokens from source code
fn lexer(src:&str) -> Vec<String> {
    //tokens
    let tokens: Vec<String> = vec!["tokens!".to_string(),];

    let mut x = 0;
    while x < src.len() {
        if &src[x..x+1] == "/" && &src[x+1..x+2] == "/" {
            let mut no_newline = true;
            let mut y = x + 2;
            while y < src.len() {
                if &src[y..y+1] == "\n" {
                    dbg!(&src[y+1..]);
                    no_newline = false;
                    x = y;
                    break;
                }
                y += 1;
            }
            if no_newline {
                return tokens;
            }
        }
        x += 1;
    }


    return tokens;
}

//returns line number and position a char in a string slice
fn char_position(text:&str,char_index:usize) -> (usize,usize) {
    let mut line_number = 1;
    let mut line_index = 0;
    let mut position = 1;

    let mut x:usize= 0;
    while x < char_index {
        if &text[x..x+1] == "\n" {
            line_number += 1;
            line_index = x;
        }
        x+=1;
    }

    position += char_index - line_index;


    (line_number, position)
}