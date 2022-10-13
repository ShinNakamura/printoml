use std::env;
use std::fs;
use std::collections::HashMap;

/// 第一引数は TOML
/// 第二引数は key
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("need TOML and key(s)");
        return;
    }
    let conf = &args[1];
    let conf = fs::read_to_string(conf).unwrap();
    let cm: HashMap<String,String> = toml::from_str(&conf).unwrap();
    for (i, k) in args.iter().enumerate() {
        if i < 2 {
            continue;
        }
        if let Some(v) = cm.get(k) {
            print!("{}", v);
        }
    }
} 
