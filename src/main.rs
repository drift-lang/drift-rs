use std::env;
use std::io::{stdin, stdout, Write};

pub const COMPILER_VERSION: &'static str = "Drift 0.0.1 (MADE AT Oct 2021 08, 13:41:48)";
pub const LICENSE: &'static str = "GNU General Public License GPL v3.0";

#[derive(Debug, PartialEq)]
enum IResult {
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

            let result = run(Env { mode, path, nfp });
            println!("{:?}", result);
        }
        _ => println!("{}", usage()),
    }
}

fn run(env: Env) -> IResult {
    if env.mode == IMode::Repl {
        repl();
    }
    IResult::Done
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

fn repl() {
    let mut p = 1;
    
    loop {
        print!("{:03} > ", p);

        let mut line = String::new();
        stdout().flush().expect("Failed to flush the screen!");
        stdin().read_line(&mut line).expect("Failed to read line!");

        if line.trim_end().len() > 0 {
            print!("EVAL: {}", line);
        }
        p += 1;
    }
}
