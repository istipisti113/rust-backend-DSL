use std::collections::{HashMap, HashSet};

use crate::parser::functions::FUNCTIONS;
mod functions;

pub static KEYWORDS: [&str; 1] = [
    "let",
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    Let
}

trait GetText {
    fn get_text(&self)-> Option<&String>;
    fn into_text(self)-> Option<String>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
pub enum Token {
    Plus,
    EqualEqual,
    NotEqual,
    Equal,
    Number(u32),
    Keyword(Keyword),
    Identifier(String),
    Function(String),
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

//simple tokenization for now
pub fn tokenize(code: String)-> Vec<Vec<Token>>{
    let mut tokenized : Vec<Vec<Token>> = vec![];
    for line in code.split("\n"){
        let mut char_indices = line.char_indices().peekable();
        let mut tokens : Vec<Token> = vec![];
        while let Some((pos, ch)) = char_indices.next() {
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
                        if FUNCTIONS.contains_key(&(character.to_string()+&s)){
                            Token::Function(character.to_string()+&s)
                        } else {
                            matchkeyword(character.to_string()+&s)
                        }
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

pub fn evaluate(code: Vec<Vec<Token>>){
    let mut variables: HashMap<String, Variable> = HashMap::new();
    println!("code ran: {:?}\n", code);
    println!("output: ");
    for line in code.into_iter(){
        if line[0] == Token::Keyword(Keyword::Let){
            match &line[1] {
                Token::Identifier(ident) => {
                    variables.insert(ident.clone(), Variable::String(line[3].clone().into_text().unwrap()));
                }
                _ => {}
            }
        }
        else {
            match &line[0] {
                Token::Function(func) => {
                    let mut vars: Vec<Variable> = vec![];
                    let err = String::from("some error occured with the params of the function");
                    line.iter().skip(1).take(line.iter().len()).for_each(|token|{

                        if let Some(Token::StringLiteral(str)) = Some(token){//string literals
                            vars.push(Variable::String(str.to_string()));
                        }else if variables.contains_key(token.get_text().unwrap_or_else(||{ //values of variables
                            dbg!(token);
                            &err
                        })){
                            //println!("{}", token.get_text().unwrap());
                            vars.push(
                                //variables.get(token.get_text().unwrap().to_string())
                                //variables.get(&String::new()).take().unwrap()
                                variables[&token.clone().into_text().unwrap()].clone()
                            );
                        }
                    });
                    FUNCTIONS.get(func).unwrap()(vars);
                }
                _ => {}
            }
        }
    }
    println!("\nvariable dump:");
    dbg!(variables);
}
