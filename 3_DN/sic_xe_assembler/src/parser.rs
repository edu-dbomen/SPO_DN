use crate::mnemonics::Mnemonic;

#[derive(Debug)]
pub struct ParserResult {
    pub label: String,
    pub mnemonic: String,
    pub operands: Vec<String>,
    pub extended: bool,
}

/// returns handled mnemonic
pub fn parse_and_handle_extended(parser_result: &mut ParserResult, mnemonic: String) -> String {
    if mnemonic.chars().nth(0).unwrap() == '+' {
        parser_result.extended = true;
        return mnemonic[1..].to_string();
    }
    mnemonic
}

pub fn parse(tokens: Vec<Vec<String>>) -> Vec<ParserResult> {
    tokens
        .into_iter()
        .map(|token| {
            let mut parser_result = ParserResult {
                label: String::new(),
                mnemonic: String::new(),
                operands: Vec::new(),
                extended: false,
            };

            let mut first_token = token.get(0).expect("Parser received empty token!").clone();
            first_token = parse_and_handle_extended(&mut parser_result, first_token);

            match Mnemonic::parse(&first_token) {
                Some(_) => {
                    parser_result.mnemonic = first_token.clone();
                    parser_result.operands = token[1..].to_vec();
                }
                None => {
                    parser_result.label = token[0].clone();

                    let mut mnemonic = token.get(1).expect("Parser received label only!").clone();
                    mnemonic = parse_and_handle_extended(&mut parser_result, mnemonic);
                    if Mnemonic::parse(&mnemonic).is_none() {
                        panic!("Parser received no mnemonic!");
                    }
                    parser_result.mnemonic = mnemonic.clone();

                    parser_result.operands = token[2..].to_vec();
                }
            }

            parser_result
        })
        .collect()
}
