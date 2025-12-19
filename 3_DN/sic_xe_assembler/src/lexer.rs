use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[rustfmt::skip]
pub fn lexer(file_reader: BufReader<File>) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = vec![];

    for line in file_reader.lines() {
        let line = line.expect("Could not read line");

        let line_arr: Vec<String> = line
            .split_whitespace()             // split by whitespace
            .flat_map(|s| s.split(','))     // split by ',' and flatten into single array
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())      // filter out empty strings
            .map(|s| s.to_string())
            .take_while(|s| s != ".")       // remove comments
            .collect();

        if !line_arr.is_empty() {
            res.push(line_arr);
        }
    }

    res
}
