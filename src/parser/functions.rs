use std::collections::HashMap;
use std::sync::LazyLock;

use crate::parser::{GetText, Variable};

type Func = fn(Vec<Variable>)-> ();

pub static FUNCTIONS :LazyLock<HashMap<String, Func>> = LazyLock::new(||{
    let mut m: HashMap<String, Func> = HashMap::new();

    m.insert("print".to_string(), |params|{
        println!("{}", params.iter().map(|param| param.get_text().unwrap().as_ref()).collect::<Vec<&str>>().join(", "))
    });

    m
});
