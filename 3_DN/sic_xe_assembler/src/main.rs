use std::{env, fs::OpenOptions, io::BufReader, process::exit};

mod mnemonics;

mod lexer;
mod parser;
// mod symbol_resolver;
// mod code_generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid arguments. Use {} <file_name>", args[0]);
        exit(1);
    }

    let file_name = &args[1];
    let file_reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(file_name)
            .expect("Could not open file"),
    );

    // .asm lines -> Lexer
    let lexer_result = lexer::lexer(file_reader);

    // Lexer -> Parser
    let parser_result = parser::parse(lexer_result);

    // Parser -> Symbol resolver

    // Symbol resolver -> Code generator

    // Code generator -> .obj code
}
