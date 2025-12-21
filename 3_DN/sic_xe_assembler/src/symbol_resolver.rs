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
    macro_expansion_counter: u32,
}

impl SymbolResolver {
    pub fn new() -> Self {
        Self {
            starting_location: 0,
            locctr: 0,
            sym_tab: HashMap::new(),
            sym_res: vec![],
            base_value: None,
            macro_expansion_counter: 0,
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
                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr,
                            instruction: token.clone(),
                            byte_code: 0,
                            byte_code_size: 0,
                        });
                        break;
                    }
                    Directive::Org => {
                        self.locctr = token.operands[0].parse::<u32>().unwrap();
                        0
                    }
                    Directive::Equ => {
                        let value: u32 = token.operands[0]
                            .parse::<u32>()
                            .expect("EQU only supports constants!");
                        self.sym_tab.insert(token.label.clone(), value);

                        0
                    }
                    Directive::Base => 0,
                    Directive::Nobase => 0,
                    Directive::Resb => token.operands[0].parse::<u32>().unwrap(),
                    Directive::Resw => token.operands[0].parse::<u32>().unwrap() * 3,
                    Directive::Byte => 1,
                    Directive::Word => 3,
                    Directive::If => {
                        let instruction_comp = ParserResult {
                            label: token.label.clone(),
                            mnemonic: "COMP".to_string(),
                            operands: vec![token.operands[0].clone()],
                            extended: false,
                        };
                        let instruction_jeq = ParserResult {
                            label: "".to_string(),
                            mnemonic: "JEQ".to_string(),
                            operands: vec![token.operands[1].clone()],
                            extended: false,
                        };

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr,
                            instruction: instruction_comp,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 3,
                            instruction: instruction_jeq,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.locctr += 6;
                        continue;
                    }
                    Directive::Mod => {
                        let mod_res1 = format!("__mod_res1_c{}", self.macro_expansion_counter);
                        self.sym_tab.insert(mod_res1.clone(), original_locctr + 21);
                        let mod_res2 = format!("__mod_res2_c{}", self.macro_expansion_counter);
                        self.sym_tab.insert(mod_res2.clone(), original_locctr + 24);
                        let mod_end = format!("__mod_end_c{}", self.macro_expansion_counter);
                        self.sym_tab.insert(mod_end.clone(), original_locctr + 27);

                        let instruction_sta1 = ParserResult {
                            label: token.label.clone(),
                            mnemonic: "STA".to_string(),
                            operands: vec![mod_res1.clone()],
                            extended: false,
                        };
                        let instruction_div = ParserResult {
                            label: "".to_string(),
                            mnemonic: "DIV".to_string(),
                            operands: vec![token.operands[0].clone()],
                            extended: false,
                        };
                        let instruction_mul = ParserResult {
                            label: "".to_string(),
                            mnemonic: "MUL".to_string(),
                            operands: vec![token.operands[0].clone()],
                            extended: false,
                        };
                        let instruction_sta2 = ParserResult {
                            label: "".to_string(),
                            mnemonic: "STA".to_string(),
                            operands: vec![mod_res2.clone()],
                            extended: false,
                        };
                        let instruction_lda = ParserResult {
                            label: "".to_string(),
                            mnemonic: "LDA".to_string(),
                            operands: vec![mod_res1.clone()],
                            extended: false,
                        };
                        let instruction_sub = ParserResult {
                            label: "".to_string(),
                            mnemonic: "SUB".to_string(),
                            operands: vec![mod_res2.clone()],
                            extended: false,
                        };
                        let instruction_j = ParserResult {
                            label: "".to_string(),
                            mnemonic: "J".to_string(),
                            operands: vec![mod_end.clone()],
                            extended: false,
                        };
                        let instruction_resw1 = ParserResult {
                            label: mod_res1.clone(),
                            mnemonic: "RESW".to_string(),
                            operands: vec!["1".to_string()],
                            extended: false,
                        };
                        let instruction_resw2 = ParserResult {
                            label: mod_res2.to_string(),
                            mnemonic: "RESW".to_string(),
                            operands: vec!["1".to_string()],
                            extended: false,
                        };
                        let instruction_filler = ParserResult {
                            label: mod_end.to_string(),
                            mnemonic: "ADD".to_string(),
                            operands: vec!["#0".to_string()],
                            extended: false,
                        };

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr,
                            instruction: instruction_sta1,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 3,
                            instruction: instruction_div,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 6,
                            instruction: instruction_mul,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 9,
                            instruction: instruction_sta2,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 12,
                            instruction: instruction_lda,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 15,
                            instruction: instruction_sub,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 18,
                            instruction: instruction_j,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 21,
                            instruction: instruction_resw1,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 24,
                            instruction: instruction_resw2,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.sym_res.push(SymbolResolverTokenResult {
                            locctr: original_locctr + 27,
                            instruction: instruction_filler,
                            byte_code: 0,
                            byte_code_size: 0,
                        });

                        self.macro_expansion_counter += 1;
                        self.locctr += 30;
                        continue;
                    }
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
                    Directive::If => {}
                    Directive::Mod => {}
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
