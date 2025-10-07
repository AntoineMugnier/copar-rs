use std::collections::HashMap;
use std::{clone, env};
use std::fs::{read_to_string, File};
use std::io::Read;
use std::str::Lines;
mod model;
mod unirecord;
use model::Model;

use unirecord::{UniRecord, UniRecordArg, UniRecordArgVariant};
enum RecordCapturingState {
    OneShot,
    Multiline,
    Ranged,
}

struct Converter {
    sequence_name: String,
    output_file: File,
    model: Model,
    capturing_state: RecordCapturingState,
    line_buffer: String,
}

impl Converter {
    pub fn new(output_file: File) -> Converter {
        Converter {
            sequence_name: String::new(),
            output_file,
            model: Model::new(),
            capturing_state: RecordCapturingState::OneShot,
            line_buffer: String::new(),
        }
    }
    fn remove_timestamp(line: &mut &str) {
        if let Some(token) = line.chars().nth(0) {
            if token == '[' {
                let split_lines = line.splitn(2, "] ");
                *line = split_lines.last().unwrap();
            }
        }
    }

    fn parse_one_shot_record_args(line: &str) -> Vec<UniRecordArgVariant> {
        let mut uni_record_args = Vec::new();
        for args in line.split(' ').collect::<Vec<&str>>().windows(2).step_by(2) {
            let arg_name = args[0];
            let arg_val = args[1];
            println!("{arg_name} {arg_val}");
            let uni_record_arg = UniRecordArgVariant::from(arg_name, arg_val).unwrap();
            uni_record_args.push(uni_record_arg);
        }
        uni_record_args
    }

    fn handle_single_line_record(&mut self) {
        let record_str_vec: Vec<&str> = self.line_buffer.splitn(2, ' ').collect();

        let name = String::from(record_str_vec[0]);
        let args = Self::parse_one_shot_record_args(record_str_vec[1]);
        let uni_record = UniRecord::new(name, args);

        self.model.add_record(uni_record);
    }

    fn one_shot_state(&mut self, line: &str) {
        let tokens: Vec<&str> = line.splitn(2, [' ']).collect();
        match tokens[0] {
            "#=" => {
                let sub_tokens: Vec<&str> = tokens[1].split(" =#").collect();
                self.line_buffer = sub_tokens[0].to_string();
                assert!(sub_tokens.len() <= 2);
                if sub_tokens.len() == 2 {
                    self.handle_single_line_record();
                } else {
                    self.capturing_state = RecordCapturingState::Multiline;
                }
            }
            "#[" => {
                self.line_buffer = String::new();
                self.line_buffer = tokens[1].to_string();
                self.capturing_state = RecordCapturingState::Ranged;
            }
            _ => {
                // Unparsable line, skip
            }
        }
    }

    fn multiline_capture_state(&mut self, line: &str) {
        let tokens: Vec<&str> = line.split(" =#").collect();

        self.line_buffer += " ";
        self.line_buffer += tokens[0];

        if tokens.len() == 2 {
            self.handle_single_line_record();
            self.capturing_state = RecordCapturingState::OneShot;
        }
    }

    fn ranged_capture_state(&mut self, line: &str) {
        let tokens: Vec<&str> = line.split("#- ").collect();
        if tokens.len() == 2 {
            self.line_buffer += " ";
            self.line_buffer += tokens[1];
            return;
        }

        let tokens: Vec<&str> = line.split(" #]").collect();
        if tokens.len() == 2 {
            let record_name = tokens[0];
            let previous_record_name = self.line_buffer.split(" ").next().unwrap();
            assert_eq!(record_name, previous_record_name);
            self.handle_single_line_record();
            self.capturing_state = RecordCapturingState::OneShot;
        }
    }
    fn is_end_line(&mut self, line: &str) -> bool {
        let tokens: Vec<&str> = line.split(" :#").collect();

        assert!(tokens.len() <= 2);

        if tokens.len() == 2 {
            let sequence_name = tokens[0];
            assert_eq!(sequence_name, self.sequence_name);
            return true;
        }

        return false;
    }

    fn move_to_begin_token(&mut self, lines: &mut Lines) {
        for mut line in lines {
            Self::remove_timestamp(&mut line);
            if line.len() > 0 {
                let tokens: Vec<&str> = line.split("#: ").collect();
                if tokens.len() == 2 {
                    self.sequence_name = tokens[1].to_string();
                    return;
                }
            }
        }
    }

    fn parse_lines(&mut self, input_file_buffer: String) {
        let mut lines_it = input_file_buffer.lines();

        self.move_to_begin_token(&mut lines_it);

        for mut line in lines_it {
            Self::remove_timestamp(&mut line);
            if self.is_end_line(&line) {
                return;
            }

            if line.len() > 0 {
                match self.capturing_state {
                    RecordCapturingState::OneShot => self.one_shot_state(line),
                    RecordCapturingState::Multiline => self.multiline_capture_state(line),
                    RecordCapturingState::Ranged => self.ranged_capture_state(line),
                }
            }
        }
    }

    pub fn handle_file(&mut self, mut input_file: File) {
        let mut input_file_buffer = String::new();
        input_file.read_to_string(&mut input_file_buffer).unwrap();
        self.parse_lines(input_file_buffer);
        self.model.compute(&mut self.output_file, self.sequence_name.clone());
    }
}
fn main() {
    let mut args = env::args();
    let _path = args.next();
    let input_file_path = args.next().unwrap();
    let input_file = File::open(input_file_path).unwrap();

    let output_file_path = args.next().unwrap();
    let output_file = File::create(output_file_path).unwrap();

    let mut converter = Converter::new(output_file);
    converter.handle_file(input_file);
}

#[cfg(test)]
mod test {
    use crate::Converter;
    use std::{fs::File, io::Write};

    #[test]
    fn test_gen() {
        let test_file_content = "
the begining\n\
[   62.996337] #: test_sequence
[   62.996339] #= CMD52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
[   64.049750] #= CMD53 write: bool(true) fn: u8(1) add: x32(0x01043) inc: bool(true)
[   64.054214] data: x8([0x80,0x05]) =#
Just trash
[   62.996339] #= CMD52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
[   58.432667] #[ READ_EFUSE
[   58.437030] #- offset: x16(0x5ea) efuse_start: x8(0x0) size: u8(1) read_efuse_cnt: u32(10000) efuse_ctrl: x8(0x30) dv_sel: id(DDV)\n
[   58.439921] #- map_ptr: x64(00000000da5708c1)
[   64.592339] I am not a record
Me neither
[   64.921023] READ_EFUSE #]
[   64.921023] test_sequence :#
[   62.996339] #= CMD52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#\n";

        let test_files_dir = "/tmp".to_string();
        let test_input_file_path = test_files_dir.clone() + "/test_input_file";

        let mut test_input_file = File::create(test_input_file_path.clone()).unwrap();
        test_input_file
            .write_all(test_file_content.as_bytes())
            .unwrap();
        drop(test_input_file);

        let test_input_file = File::open(test_input_file_path.clone()).unwrap();

        let test_output_file_path = test_files_dir.clone() + "/test_output_file";
        let test_output_file = File::create(test_output_file_path).unwrap();

        let mut converter = Converter::new(test_output_file);
        converter.handle_file(test_input_file);
    }
}
