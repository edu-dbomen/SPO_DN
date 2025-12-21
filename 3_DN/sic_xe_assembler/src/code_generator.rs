use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{
    mnemonics::Directive,
    symbol_resolver::{SymbolResolverResult, SymbolResolverTokenResult},
};

const NEW_BYTE_CODE_THRESHOLD: u32 = 0x30;
struct TRecordState {
    used: bool,
    locctr: u32,
    current_byte_code_size: u32,
    ascii_byte_code: String,
}

struct MRecord {
    address: u32,
    len: u32,
}

pub fn generate_code(file_name: &String, tokens: &mut SymbolResolverResult) {
    generate_lst(file_name, tokens);
    generate_obj(file_name, tokens);
}

fn generate_lst(file_name: &String, tokens: &SymbolResolverResult) {
    let mut path = PathBuf::from(file_name);
    path.set_extension("lst");
    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.to_string_lossy().into_owned())
            .expect("Could not open file"),
    );

    for token in tokens.sym_res.iter() {
        let locctr = token.locctr + tokens.starting_location;
        let byte_code_format = format!(
            "{:8}",
            format!(
                "{:0width$x}",
                token.byte_code,
                width = (token.byte_code_size / 4) as usize
            )
        );

        writeln!(
            file,
            "{:06x}  {}  {:12}  {:6}  {:?}",
            locctr,
            byte_code_format,
            token.instruction.label,
            token.instruction.mnemonic,
            token.instruction.operands
        )
        .expect("Could not write line");
    }

    file.flush().expect("Could not flush");
}

// ************************************************************************************************

fn generate_obj(file_name: &String, tokens: &mut SymbolResolverResult) {
    let mut path = PathBuf::from(file_name);
    path.set_extension("obj");
    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.to_string_lossy().into_owned())
            .expect("Could not open file"),
    );

    generate_header_record(&mut file, tokens);

    let mut m_records: Vec<MRecord> = vec![];
    let mut t_record_state: TRecordState = TRecordState {
        used: false,
        locctr: 0,
        current_byte_code_size: 0,
        ascii_byte_code: String::new(),
    };
    for token in tokens.sym_res.iter() {
        if needs_relocation(token) {
            m_records.push(MRecord {
                address: tokens.starting_location + token.locctr + 1,
                len: 0x05,
            });
        }

        // if new t record state (not currently being built)
        if !t_record_state.used {
            t_record_state.used = true;
            t_record_state.locctr = token.locctr + tokens.starting_location;
            t_record_state.current_byte_code_size = 0;
            t_record_state.ascii_byte_code = String::new();
        }

        // write t record if RESB or RESW
        match token.instruction.mnemonic.parse::<Directive>() {
            Ok(directive) => match directive {
                Directive::Resb => {
                    generate_text_record(&mut file, &t_record_state);
                    t_record_state.used = false;
                    continue;
                }
                Directive::Resw => {
                    generate_text_record(&mut file, &t_record_state);
                    t_record_state.used = false;
                    continue;
                }
                _ => {}
            },
            Err(_) => {}
        }

        // add new byte code
        let new_byte_code = format!(
            "{:0width$x}",
            token.byte_code,
            width = (token.byte_code_size / 4) as usize
        );
        t_record_state.current_byte_code_size += token.byte_code_size / 8;
        t_record_state.ascii_byte_code += &new_byte_code;

        // write t record if too big
        if t_record_state.current_byte_code_size >= NEW_BYTE_CODE_THRESHOLD {
            generate_text_record(&mut file, &t_record_state);
            t_record_state.used = false;
        }
    }
    // generate last t record
    if t_record_state.used {
        generate_text_record(&mut file, &t_record_state);
        t_record_state.used = false;
    }

    // generate m records
    generate_m_records(&mut file, m_records);

    generate_end_record(&mut file, tokens.starting_location);
}

fn generate_header_record(file: &mut BufWriter<File>, tokens: &mut SymbolResolverResult) {
    // check for and remove END directive and get byte code size
    // ---
    let last = tokens.sym_res.last().expect("No last token!");
    let directive = last
        .instruction
        .mnemonic
        .parse::<Directive>()
        .expect("Invalid directive!");
    if directive != Directive::End {
        panic!("END directive not present!");
    }

    let global_byte_code_size = last.locctr;
    tokens.sym_res.pop();
    // ---

    // check for and remove START directive and build H record
    // ---
    let first = tokens.sym_res.first().expect("No first token!");
    let directive = first
        .instruction
        .mnemonic
        .parse::<Directive>()
        .expect("Invalid directive!");
    if directive != Directive::Start {
        panic!("START directive not first!");
    }

    if first.instruction.label.len() > 6 {
        panic!("program label too big (>6 chars)")
    }
    writeln!(
        file,
        "H{:6}{:06x}{:06x}",
        first.instruction.label, tokens.starting_location, global_byte_code_size
    )
    .expect("Could not write line");
    tokens.sym_res.remove(0);
    // ---
}

fn generate_end_record(file: &mut BufWriter<File>, starting_location: u32) {
    writeln!(file, "E{:06x}", starting_location).expect("Could not write line");
}

fn generate_text_record(file: &mut BufWriter<File>, t_record_state: &TRecordState) {
    writeln!(
        file,
        "T{:06x}{:02x}{}",
        t_record_state.locctr,
        t_record_state.current_byte_code_size,
        t_record_state.ascii_byte_code
    )
    .expect("Could not write line");
}

fn generate_m_records(file: &mut BufWriter<File>, m_records: Vec<MRecord>) {
    for record in m_records.iter() {
        writeln!(file, "M{:06x}{:02x}", record.address, record.len).expect("Could not write line");
    }
}
fn needs_relocation(token: &SymbolResolverTokenResult) -> bool {
    if !token.instruction.extended {
        return false;
    }

    if token.instruction.operands.len() <= 0 {
        return false;
    }

    let mut operand = token.instruction.operands[0].clone();
    if operand.chars().nth(0).unwrap() == '#' || operand.chars().nth(0).unwrap() == '@' {
        operand = operand[1..].to_string();
    }
    match operand.parse::<i32>() {
        Ok(_) => false,
        Err(_) => true,
    }
}
