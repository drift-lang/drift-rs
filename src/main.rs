use std::env;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

use drift::lexer::Lexer;

pub const COMPILER_VERSION: &'static str = "Drift 0.0.1 (MADE AT Oct 2021 08, 13:41:48)";
pub const LICENSE: &'static str = "GNU General Public License GPL v3.0";

#[derive(Debug, PartialEq)]
pub enum IResult {
    Done,
}

#[derive(Debug, PartialEq)]
enum IMode {
    Repl,
    Token,
    Op,
    Tb,
    None,
}

#[derive(Debug)]
pub struct Env {
    mode: IMode,
    path: String,
    nfp: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    let parse = |x: String| -> IMode {
        match x.as_str() {
            "repl" => IMode::Repl,
            "token" => IMode::Token,
            "op" => IMode::Op,
            "tb" => IMode::Tb,

            _ => IMode::None,
        }
    };
    match len {
        2..=3 => {
            let arg = args.get(if len == 2 { 1 } else { 2 }).unwrap();

            let mode = parse(arg.clone());
            let mut fp: Option<&String> = args.get(1);

            if len == 2 && mode != IMode::None {
                fp = None;
            }
            let path = if fp == None {
                String::new()
            } else {
                fp.unwrap().clone()
            };
            let nfp = path.is_empty();
            execute(Env { mode, path, nfp });
        }
        _ => println!("{}", usage()),
    }
}

fn usage() -> String {
    format!(
        "
            Drift Interpreter With Rust!

usage: drift [FILE(.ft)] <option>

command:
  repl      enter read-eval-print-loop mode
  token     show lexical token list
  op        show bytecode
  tb        after exec, show environment mapping

version: {}
license: {}
          @ bingxio - bingxio@qq.com",
        COMPILER_VERSION, LICENSE
    )
}

fn execute(env: Env) {
    if env.mode == IMode::Repl {
        repl();
    } else {
        if env.nfp {
            panic!("Specify a drift program source file");
        }
        if !env.path.ends_with(".ft") {
            panic!("Specify a file ending in an `.ft` suffix");
        }
        match read_to_string(env.path) {
            Ok(code) => {
                let result = evaluate(code);
                println!("{:?}", result);
            }
            Err(error) => panic!("Failed to read file: {}", error),
        }
    }
}

fn repl() {
    let mut p = 1;
    loop {
        print!("{:03} > ", p);

        let mut line = String::new();
        stdout().flush().expect("Failed to flush the screen!");
        stdin().read_line(&mut line).expect("Failed to read line!");

        if line.trim_end().len() > 0 {
            line.pop();
            evaluate(line);
        }
        p += 1;
    }
}

pub fn evaluate(code: String) -> IResult {
    let tokens = Lexer::new(code).lexical();
    println!("{:?}", tokens);
    IResult::Done
}
