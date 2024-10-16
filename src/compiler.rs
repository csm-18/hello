//compiler.rs : contains actual compiler code

//converts hello code into assembly and returns it
pub fn compile(src:&str) -> String {
    //output(assembly code)
    let output: String = "output!".to_owned();

    //make tokens from src
    let tokens = lexer(src);
    dbg!(tokens);

    return output;
}

//creates tokens from source code
fn lexer(src:&str) -> Vec<String>{
    //tokens
    let tokens: Vec<String> = vec!["tokens!".to_string(),];


    return tokens;
}