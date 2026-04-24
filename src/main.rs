use std::{any::Any, collections::HashMap, env, fs, vec};

static KEYWORDS: [&str; 1] = [
    "let",
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Keyword {
    Let
}

trait GetText {
    fn get_text(&self)-> Option<&String>;
    fn into_text(self)-> Option<String>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Variable {
    String(String),
    Integer(i32)
}

impl GetText for Variable{
    fn get_text(&self)-> Option<&String>{
        match self {
            Variable::String(value) => Some(&value),
            _ => None
        }
    }

    fn into_text(self)-> Option<String>{
        match self {
            Variable::String(value) => Some(value),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    Plus,
    EqualEqual,
    NotEqual,
    Equal,
    Number(u32),
    Keyword(Keyword),
    Identifier(String),
    StringLiteral(String),
    Invalid,
}

impl GetText for Token {
    fn get_text(&self)-> Option<&String>{
        match self {
            Token::Identifier(value) => Some(&value),
            Token::StringLiteral(value) => Some(&value),
            _ => None
        }
    }

    fn into_text(self)-> Option<String>{
        match self {
            Token::Identifier(value) => Some(value),
            Token::StringLiteral(value) => Some(value),
            _ => None
        }
    }
}

fn main() {
    println!("rust backend dsl");
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(filenev) = args.get(0){
        match fs::read_to_string(filenev) {
            Ok(code)=>{
                let tokenized = tokenize(code);
                evaluate(tokenized);
            }
            Err(e)=>{println!("{}", e)}
        };
    }
}

//simple tokenization for now
fn tokenize(code: String)-> Vec<Vec<Token>>{
    let mut tokenized : Vec<Vec<Token>> = vec![];
    for line in code.split("\n"){
        let mut char_indices = line.char_indices().peekable();
        let mut tokens : Vec<Token> = vec![];
        while let Some((pos, ch)) = char_indices.next() {
            println!("--{}--", ch);
            let token = match ch {
                '+' => Token::Plus,
                '=' => {
                    match char_indices.next_if_eq(&(pos+1, '=')) {
                        Some(_equals) => Token::EqualEqual,
                        None => Token::Equal,
                    }
                },
                '!' => {
                    match char_indices.next_if_eq(&(pos+1, '=')) {
                        Some(_equals) => Token::NotEqual,
                        None => Token::Invalid,
                    }
                },

                '\"' => {
                    let mut last_matched: char = '\0';
                    let s: String = char_indices
                        .by_ref()
                        .take_while(|(_pos, c)| { 
                            last_matched = *c;
                            *c != '"' 
                        })
                        .map(|(_pos, c)| { c })
                        .collect();

                    match last_matched {
                        '"' => Token::StringLiteral(s),
                        _ => Token::Invalid,
                    }
                },
                character => {
                    if character.is_ascii() && character!=' '{
                        let s: String = char_indices
                            .by_ref()
                            .take_while(|(_pos, c)| { 
                                *c != ' ' && c.is_ascii()
                            })
                            .map(|(_pos, c)| { c })
                            .collect();
                        matchkeyword(character.to_string()+&s)
                    } else {
                        Token::Invalid
                    }
                },
            };
            tokens.push(token);
        }
        tokenized.push(tokens.into_iter().filter(|token| *token != Token::Invalid).collect());
    }
    tokenized.into_iter().filter(|line|*line!=vec![]).collect()
}

fn matchkeyword(token: String) -> Token {
    match token.as_ref() {
        "let" => Token::Keyword(Keyword::Let),
        _ => Token::Identifier(token)
    }
}

fn evaluate(code: Vec<Vec<Token>>){
    let mut variables: HashMap<String, Variable> = HashMap::new();
    println!("{:?}", code);
    for line in code.into_iter(){
        if line[0] == Token::Keyword(Keyword::Let){
            match &line[1] {
                Token::Identifier(ident) => {
                    variables.insert(ident.clone(), Variable::String(line[3].clone().into_text().unwrap()));
                }
                _ => {}
            }
        }
    }
    dbg!(variables);
}
