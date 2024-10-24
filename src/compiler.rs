/// Contains compile() and other related functions.

use std::process::exit;

pub fn compile(src:&str) -> String {
    //output(assembly code)
    let output: String = "output!".to_owned();

    let tokens = lexer(src);
    dbg!(tokens);

    output
}


/* Lexical Analysis Start */
#[derive(Debug)]
enum Token {
    START,
    STR_LIT(String),     //string literal
    LEFT_PAREN(String),  //left parenthesis
    RIGHT_PAREN(String), //right parenthesis
    LEFT_BRACE(String),
    RIGHT_BRACE(String),
    FUN_KEYWORD, //fun keyword
    ID(String),          //identifier
    SPACE,               //whitespace char (' ')
    NEWLINE,             //newline char ('\n')
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
        } else if &src[x..x+1] == "(" || &src[x..x+1] == ")" || &src[x..x+1] == "{" || &src[x..x+1] == "}" || &src[x..x+1] == " " || &src[x..x+1] == "\n" {
            match &src[x..x + 1] {
                "(" =>  tokens.push(Token::LEFT_PAREN("(".to_string())),
                ")" =>  tokens.push(Token::RIGHT_PAREN(")".to_string())),
                "{" =>  tokens.push(Token::LEFT_BRACE("{".to_string())),
                "}" =>  tokens.push(Token::RIGHT_BRACE("}".to_string())),
                " " =>  tokens.push(Token::SPACE),
                "\n" => tokens.push(Token::NEWLINE),
                _ => exit(1),
            }
        }else if valid_word(&src[x..x+1]) {
            let mut word: String = "".to_string();
            let mut temp= "".to_string();

            let mut start = true;
            let mut y = x;
            while start || valid_word(&temp.clone()) {
                start = false;
                word = temp.clone();

                temp.insert_str(temp.len(),&src[y..y+1]);
               y += 1;
            }

            match word.as_str() {
                "fun" => {
                    tokens.push(Token::FUN_KEYWORD);
                    x  = y - 1;
                },
                _ => {
                    if &word[0..1] == "0" || &word[0..1] == "1" || &word[0..1] == "2" || &word[0..1] == "3" || &word[0..1] == "4"  || &word[0..1] == "5" || &word[0..1] == "6" || &word[0..1] == "7" || &word[0..1] == "8" || &word[0..1] == "9"  {
                        let (line, error_at) = char_position(&src,x);
                        println!("Token Error at {error_at} on line {line}.");
                        exit(1);
                    }else {
                        tokens.push(Token::ID(word));
                        x = y - 1;
                    }
                }
            }

        }

        x += 1;
    }

    tokens.push(Token::END);
    return tokens;
}

fn valid_word(word: &str) -> bool {
    let mut valid = false;
    for char in word.chars() {
        match char {
            '_'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z' => valid = true,
            _ => {
                valid = false;
                return valid;
            },
        }
    }
    return valid;
}
/* Lexical Analysis End */

//find line number and position of a char in a string slice
fn char_position(source_code:&str,char_index:usize) -> (usize,usize) {
    let mut line_number = 1;
    let mut line_index = 0;
    let mut position = 0;

    let mut x:usize= 0;
    while x < char_index {
        if &source_code[x..x+1] == "\n" {
            line_number += 1;
            line_index = x;
        }
        x+=1;
    }

    position = char_index - line_index;

    (line_number, position)
}

