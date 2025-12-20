use std::collections::HashMap;

use crate::{
    mnemonics::{Directive, Opcode},
    parser::ParserResult,
};

#[derive(Debug)]
struct SymbolResolverTokenResult {
    locctr: u32,
    instruction: ParserResult,
    byte_code: u32,
}
pub struct SymbolResolverResult {
    pub starting_location: u32,
    pub sym_res: Vec<SymbolResolverTokenResult>,
}

pub struct SymbolResolver {
    locctr: u32,
    sym_tab: HashMap<String, u32>, // symbol, locctr
    sym_res: Vec<SymbolResolverTokenResult>,
    starting_location: u32,
}

impl SymbolResolver {
    pub fn new() -> Self {
        Self {
            starting_location: 0,
            locctr: 0,
            sym_tab: HashMap::new(),
            sym_res: vec![],
        }
    }

    pub fn resolve_symbols(&mut self, tokens: Vec<ParserResult>) -> SymbolResolverResult {
        self.first_pass(tokens);
        for res in self.sym_res.iter() {
            println!("{:?}", res);
        }
        SymbolResolverResult {
            starting_location: 0,
            sym_res: vec![],
        }

        // self.second_pass();
        //
        // SymbolResolverResult {
        //     starting_location: self.starting_location,
        //     sym_res: std::mem::take(&mut self.sym_res),
        // }
    }

    /// fill symtab + sym_res (with no byte code yet)
    fn first_pass(&mut self, tokens: Vec<ParserResult>) {
        for token in tokens.iter() {
            let original_locctr = self.locctr;

            // if not exists => create new entry in sym_tab
            if self.sym_tab.contains_key(&token.label) {
                panic!("Duplicate symbols! {}", token.label);
            }
            if !token.label.is_empty() {
                self.sym_tab.insert(token.label.clone(), self.locctr);
            }

            // if opcode =>
            if let Ok(opcode) = token.mnemonic.parse::<Opcode>() {
                self.locctr += match opcode.format() {
                    crate::mnemonics::Format::F1 => 1,
                    crate::mnemonics::Format::F2 => 2,
                    crate::mnemonics::Format::F3_4 => {
                        if !token.extended {
                            3
                        } else {
                            4
                        }
                    }
                }
            }
            // if directive =>
            else if let Ok(directive) = token.mnemonic.parse::<Directive>() {
                self.locctr += match directive {
                    Directive::Start => {
                        self.starting_location = token.operands[0].parse::<u32>().unwrap();
                        0
                    }
                    Directive::End => {
                        break;
                    }
                    Directive::Org => token.operands[0].parse::<u32>().unwrap(),
                    Directive::Equ => 0,
                    Directive::Base => 0,
                    Directive::Nobase => 0,
                    Directive::Resb => token.operands[0].parse::<u32>().unwrap(),
                    Directive::Resw => token.operands[0].parse::<u32>().unwrap() * 3,
                    Directive::Byte => token.operands[0].parse::<u32>().unwrap(),
                    Directive::Word => token.operands[0].parse::<u32>().unwrap() * 3,
                }
            }
            // else
            else {
                panic!("Got a mnemonic that is not a mnemonic or directive");
            }

            // add entry to sym_res
            self.sym_res.push(SymbolResolverTokenResult {
                locctr: original_locctr,
                instruction: token.clone(),
                byte_code: 0,
            });
        }
    }

    /// fill sym_res with byte code
    fn second_pass(&self) {}
}
