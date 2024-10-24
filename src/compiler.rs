//compiler.rs : contains actual compiler code

use std::process::exit;

//converts hello code into assembly and returns it
pub fn compile(src:&str) -> String {
    //output(assembly code)
    let output: String = "output!".to_owned();

    let tokens = lexer(src);
    dbg!(tokens);
    output
}




//create tokens from source code
#[derive(Debug)]
enum Token {
    START,
    STR_LIT(String),     //string literal
    LEFT_PAREN(String),  //left parenthesis
    RIGHT_PAREN(String), //right parenthesis
    LEFT_BRACE(String),
    RIGHT_BRACE(String),
    KEYWORD(String),
    ID(String),          //identifier
    END,
}
fn lexer(src:&str) -> Vec<Token> {


    //tokens
    let mut tokens: Vec<Token> = vec![Token::START,];

    let mut x = 0;
    while x < src.len() {
        if &src[x..x+1] == "`" {
            let mut no_end_backtick = true;
            let mut y = x + 1;
            while y < src.len() {
                if &src[y..y+1] == "`" && &src[y-1..y] != "\\" {
                    no_end_backtick = false;
                    let token = Token::STR_LIT(src[x..y+1].to_string());
                    tokens.push(token);
                    x = y;
                    break;
                }
                y += 1;
            }
            if no_end_backtick {
                let (line,error_at) = char_position(&src,x);
                println!("Token Error at {error_at} on line {line}");
                println!("String Literal Not Terminated!");
                exit(1);
            }
        } else if &src[x..x+1] == "/" && &src[x+1..x+2] == "/" {
            let mut no_newline = true;
            let mut y = x + 2;
            while y < src.len() {
                if &src[y..y+1] == "\n" {
                    no_newline = false;
                    x = y;
                    break;
                }
                y += 1;
            }
            if no_newline {
                return tokens;
            }
        } else if &src[x..x+1] == "(" || &src[x..x+1] == ")" || &src[x..x+1] == "{" || &src[x..x+1] == "}" {
            match &src[x..x + 1] {
                "(" => tokens.push(Token::LEFT_PAREN("(".to_string())),
                ")" => tokens.push(Token::RIGHT_PAREN(")".to_string())),
                "{" => tokens.push(Token::LEFT_BRACE("{".to_string())),
                "}" => tokens.push(Token::RIGHT_BRACE("}".to_string())),
                _ => exit(1),
            }
        }
        // } else {
        //     //An identifier is made of letters
        //     let letters = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','_','0','1','2','3','4','5','6','7','8','9'];
        //     let mut token_error = true;
        //
        //     let mut temp_word = "";
        //
        //
        //     if(token_error){
        //         println!("Token Error at {error_at} on line {line}.");
        //         exit(1);
        //     }
        // }
        x += 1;
    }


    return tokens;
}

//find line number and position a char in a string slice
fn char_position(text:&str,char_index:usize) -> (usize,usize) {
    let mut line_number = 1;
    let mut line_index = 0;
    let mut position = 0;

    let mut x:usize= 0;
    while x < char_index {
        if &text[x..x+1] == "\n" {
            line_number += 1;
            line_index = x;
        }
        x+=1;
    }

    position = char_index - line_index;


    (line_number, position)
}

