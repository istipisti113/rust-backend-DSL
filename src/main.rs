use std::{env, fs};

static KEYWORDS: [&str; 1] = [
    "let",
];

enum TokenType {
    Keyword,
    Identifier,
    Operator
}

enum OperatorType {
    Plus,
    Minus,
    Equal
}

struct Token{
    Type: TokenType,
    Value: Option<String>, 
    Operator: Option<OperatorType>,
}

fn main() {
    println!("rust backend dsl");
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(filenev) = args.get(0){
        match fs::read_to_string(filenev) {
            Ok(code)=>{
                tokenize(code);
            }
            Err(e)=>{println!("{}", e)}
        };
    }
}

//simple tokenization for now
fn tokenize(code: String)-> Vec<Vec<TokenType>>{
    for token in code.split(" "){
        if KEYWORDS.contains(&token){

        }
    };
    vec![]
}

fn evaluate(code: String){

}
