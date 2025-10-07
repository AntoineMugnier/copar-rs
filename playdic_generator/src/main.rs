use std::collections::HashMap;
use std::env;
use std::fs::{read_to_string, File};
use std::io::Read;
mod unirecord;
use unirecord::{UniRecord, UniRecordArg, UniRecordArgVariant};
enum Record {
    OneShot(UniRecord),
    Multiline(UniRecord),
    Ranged(UniRecord),
}

fn remove_timestamp(line: &mut &str) {
    if line.chars().nth(0).unwrap() == '[' {
        let split_lines = line.splitn(2, "] ");
        *line = split_lines.last().unwrap();
    }
}

fn parse_one_shot_record_args(line: &str) -> Vec<UniRecordArgVariant>{
    let mut uni_record_args = Vec::new();
    for args in line.split(' ').collect::<Vec<&str>>().windows(2){
        let arg_name = args[0];
        let arg_val = args[1];
        let uni_record_arg = UniRecordArgVariant::from(arg_name, arg_val).unwrap();
        uni_record_args.push(uni_record_arg);
    }
    uni_record_args

}
fn handle_file(file: String) {
    for mut line in file.lines() {
        remove_timestamp(&mut line);
        let tokens: Vec<&str> = line.splitn(2, [' ']).collect();

        match tokens[0] {
            "#=" => {
                let tokens: Vec<&str> = tokens[1].split(" =#").collect();
                    if tokens.len() >= 1 {
                        let record_str = tokens[0];
                        let record_str_vec: Vec<&str> = record_str.splitn(2, ' ').collect();
                        let name = String::from(record_str_vec[0]);
                        let args = parse_one_shot_record_args(record_str_vec[2]);
                        let uni_record = UniRecord::new(
                            name,
                            args
                        );
                        let one_shot_record = Record::OneShot(uni_record);
                    }
                else{
                    // multi line record
                }
            }
            _ => {
                // Unparsable line, skip
            }
        }
        // let record = UniRecord{
        //     name: String::from(tokens[0])
        // };
        println!("{:?}", tokens);
    }
}

fn main() {
    let mut args = env::args();
    let _path = args.next();
    let filename = args.next().unwrap();
    let file = read_to_string(filename).unwrap();
    handle_file(file);
}

#[cfg(test)]
mod test {
    use crate::handle_file;

    #[test]
    fn test_gen() {
        let test_file = "[   62.996339]  #= CMD52 write: bool(true) fn: u8(1) add: u32(0x01043) data: u8([0x80,0x05]) =#";

        // let test_file = "[   64.049750] #= CMD53 W F1 add: 0x01040 Inc 12 Bytes data: 
        // [   64.054214] CMD53_D 0xaf,0x0,0x0,0x80,0x81,0x0,0x0,0x0,0x4,0x0,0x0,0x0=#";

        // let test_file = "[   64.671470] #[ READ_EFUSE [#
        // [   64.675139] #- READ_EFUSE offset: 0x4 efuse_start: 0x0 size: 1 read_efuse_cnt: 50000 efuse_ctrl: 0xc30 dv_sel: DDV
        // [   64.678022] #- map_ptr: 00000000f00222e5
        // [   64.921023] #] READ_EFUSE #]";

        handle_file(test_file.to_string());
    }
}
