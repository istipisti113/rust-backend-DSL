use std::{any::Any, collections::HashMap, env, fs, vec};
mod parser;

fn main() {
    println!("rust backend dsl");
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(filenev) = args.get(0){
        match fs::read_to_string(filenev) {
            Ok(code)=>{
                let tokenized = parser::tokenize(code);
                parser::evaluate(tokenized);
            }
            Err(e)=>{println!("{}", e)}
        };
    }
}

