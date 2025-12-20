use std::{collections::HashMap, str::FromStr};

use crate::{
    mnemonics::{Directive, Opcode},
    parser::ParserResult,
};

#[derive(Debug)]
pub struct SymbolResolverTokenResult {
    pub locctr: u32,
    pub instruction: ParserResult,
    pub byte_code: u32,
    pub byte_code_size: u32,
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
    base_value: Option<u32>,
}

impl SymbolResolver {
    pub fn new() -> Self {
        Self {
            starting_location: 0,
            locctr: 0,
            sym_tab: HashMap::new(),
            sym_res: vec![],
            base_value: None,
        }
    }

    pub fn resolve_symbols(&mut self, tokens: Vec<ParserResult>) -> SymbolResolverResult {
        self.first_pass(tokens);
        self.second_pass();

        SymbolResolverResult {
            starting_location: self.starting_location,
            sym_res: std::mem::take(&mut self.sym_res),
        }
    }

    /// fill symtab + sym_res (with no byte code yet)
    fn first_pass(&mut self, tokens: Vec<ParserResult>) {
        for token in tokens.iter() {
            let original_locctr = self.locctr;

            // if not exists => create new entry in sym_tab
            if !token.label.is_empty() {
                if self.sym_tab.contains_key(&token.label) {
                    panic!("Duplicate symbols! {}", token.label);
                }
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
                    Directive::Org => {
                        self.locctr = token.operands[0].parse::<u32>().unwrap();
                        0
                    }
                    Directive::Equ => 0,
                    Directive::Base => 0,
                    Directive::Nobase => 0,
                    Directive::Resb => token.operands[0].parse::<u32>().unwrap(),
                    Directive::Resw => token.operands[0].parse::<u32>().unwrap() * 3,
                    Directive::Byte => 1,
                    Directive::Word => 3,
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
                byte_code_size: 0,
            });
        }
    }

    /// fill sym_res with byte code
    fn second_pass(&mut self) {
        for res in self.sym_res.iter_mut() {
            // if opcode =>
            if let Ok(opcode) = res.instruction.mnemonic.parse::<Opcode>() {
                match opcode.format() {
                    crate::mnemonics::Format::F1 => {
                        res.byte_code = opcode as u32;

                        res.byte_code_size = 8;
                    }
                    crate::mnemonics::Format::F2 => {
                        let opcode = opcode as u32;
                        let r1 = res.instruction.operands[0].parse::<Register>().unwrap() as u32;
                        let r2 = if res.instruction.operands.get(1).is_some() {
                            res.instruction.operands[1].parse::<Register>().unwrap() as u32
                        } else {
                            0
                        };
                        res.byte_code = (opcode << 8 | r1 << 4 | r2) as u32;

                        res.byte_code_size = 16;
                    }
                    crate::mnemonics::Format::F3_4 => {
                        let mut instruction = Instruction_F3_4 {
                            opcode: opcode as u8,
                            n: true,
                            i: true,
                            x: false,
                            b: false,
                            p: false,
                            e: res.instruction.extended,
                            address: 0,
                        };

                        if let Some(operand) = res.instruction.operands.get(0) {
                            // handle x bit
                            if res.instruction.operands.get(1).is_some() {
                                instruction.x = true;
                            }

                            // resolve address + handle n,i bits
                            // ---
                            let mut is_immidiate_label = false;
                            instruction.address = match operand.parse::<i32>() {
                                Ok(op) => op,
                                Err(_) => {
                                    // immediate
                                    if operand.chars().nth(0).unwrap() == '#' {
                                        instruction.n = false;
                                        instruction.i = true;
                                        match operand[1..].parse::<i32>() {
                                            Ok(op) => op,
                                            Err(_) => {
                                                is_immidiate_label = true;
                                                *self.sym_tab.get(&operand[1..]).unwrap() as i32
                                            }
                                        }
                                    }
                                    // indirect
                                    else if operand.chars().nth(0).unwrap() == '@' {
                                        instruction.n = true;
                                        instruction.i = false;
                                        *self.sym_tab.get(&operand[1..]).unwrap() as i32
                                    }
                                    // simple
                                    else {
                                        instruction.n = true;
                                        instruction.i = true;
                                        *self.sym_tab.get(operand).unwrap() as i32
                                    }
                                }
                            };
                            // ---

                            // handle b,p bits
                            // ---
                            // handle only for non immediate and non extended
                            if !(!instruction.n && instruction.i && !is_immidiate_label
                                || instruction.e)
                            {
                                let next_pc =
                                    res.locctr + if res.instruction.extended { 4 } else { 3 };
                                let pc_difference: i32 =
                                    instruction.address as i32 - next_pc as i32;
                                let base_difference: i32 = if self.base_value.is_some() {
                                    instruction.address as i32 - self.base_value.unwrap() as i32
                                } else {
                                    0
                                };

                                // pc relative
                                if pc_difference >= -2048 && pc_difference <= 2047 {
                                    instruction.address = pc_difference;
                                    instruction.b = false;
                                    instruction.p = true;
                                }
                                // base relative
                                else if base_difference >= 0
                                    && base_difference <= 4095
                                    && self.base_value.is_some()
                                {
                                    instruction.address = base_difference;
                                    instruction.b = true;
                                    instruction.p = false;
                                } else {
                                    panic!("Could not do PC relative nor BASE relative for non extended instruction!")
                                }
                            }
                        }
                        // ---

                        // get byte code
                        // ---
                        let op6 = (instruction.opcode as u32) & 0xFC;
                        let ni = ((instruction.n as u32) << 1) | (instruction.i as u32);
                        let op_ni = op6 | ni;

                        let xbpe = ((instruction.x as u32) << 3)
                            | ((instruction.b as u32) << 2)
                            | ((instruction.p as u32) << 1)
                            | (instruction.e as u32);

                        if instruction.e {
                            let addr = (instruction.address as u32) & 0xFFFFF;
                            res.byte_code = (op_ni << 24) | (xbpe << 20) | addr;

                            res.byte_code_size = 32;
                        } else {
                            let addr = (instruction.address as u32) & 0xFFF;
                            res.byte_code = (op_ni << 16) | (xbpe << 12) | addr;

                            res.byte_code_size = 24;
                        }
                        // ---
                    }
                };
            }
            // if directive =>
            else if let Ok(directive) = res.instruction.mnemonic.parse::<Directive>() {
                match directive {
                    Directive::Start => {}
                    Directive::End => {}
                    Directive::Org => {}
                    Directive::Equ => {}
                    Directive::Base => {
                        self.base_value = Some(res.instruction.operands[0].parse::<u32>().unwrap());
                    }
                    Directive::Nobase => {
                        self.base_value = None;
                    }
                    Directive::Resb => {}
                    Directive::Resw => {}
                    Directive::Byte => {
                        res.byte_code = res.instruction.operands[0].parse::<u32>().unwrap();

                        res.byte_code_size = 8;
                    }
                    Directive::Word => {
                        res.byte_code = res.instruction.operands[0].parse::<u32>().unwrap();

                        res.byte_code_size = 24;
                    }
                }
            }
            // else
            else {
                panic!("Got a mnemonic that is not a mnemonic or directive");
            }
        }
    }
}

// ************************************************************************************************

enum Register {
    A = 0,
    X = 1,
    L = 2,
    B = 3,
    S = 4,
    T = 5,
    F = 6,
}
impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Register::*;

        match s.to_ascii_uppercase().as_str() {
            "A" => Ok(A),
            "X" => Ok(X),
            "L" => Ok(L),
            "B" => Ok(B),
            "S" => Ok(S),
            "T" => Ok(T),
            "F" => Ok(F),

            _ => Err(()),
        }
    }
}

struct Instruction_F3_4 {
    opcode: u8,
    n: bool,
    i: bool,
    x: bool,
    b: bool,
    p: bool,
    e: bool,
    address: i32, // 12, 15 or 20b
}
