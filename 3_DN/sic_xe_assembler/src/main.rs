use std::{env, fs::OpenOptions, io::BufReader, process::exit};

use crate::symbol_resolver::SymbolResolver;

mod mnemonics;

mod code_generator;
mod lexer;
mod parser;
mod symbol_resolver;

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
    let mut symbol_resolver_result = SymbolResolver::new().resolve_symbols(parser_result);

    // Symbol resolver -> Code generator
    code_generator::generate_code(file_name, &mut symbol_resolver_result);
}
