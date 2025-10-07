use std::collections::HashMap;
use std::{clone, env};
use std::fs::{read_to_string, File};
use std::io::Read;
use std::str::Lines;
mod model;
mod c_generation;
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
    input_file_buffer:Option<String>,
    model: Model,
    capturing_state: RecordCapturingState,
    line_buffer: String,
}

impl Converter {
    pub fn new(mut input_file: File) -> Converter {

        let mut input_file_buffer = String::new();
        input_file.read_to_string(&mut input_file_buffer).unwrap();

        Converter {
            sequence_name: String::new(),
            input_file_buffer: Some(input_file_buffer),
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
            println!("{} + {}", arg_name, arg_val);
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
       
        // Add space if last line was uncut
        if self.line_buffer.chars().last().unwrap() == ')'{
            self.line_buffer += " ";
        }
        self.line_buffer += tokens[0];

        if tokens.len() == 2 {
            println!("{}", self.line_buffer);
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
    fn is_end_line( line: &str, sequence_name: &String) -> bool {
        let tokens: Vec<&str> = line.split(" :#").collect();

        assert!(tokens.len() <= 2);

        if tokens.len() == 2 {
            let sequence_name = tokens[0];
            assert_eq!(sequence_name, sequence_name);
            return true;
        }

        return false;
    }

    fn move_to_begin_token(lines: &mut Lines) -> String {
        for mut line in lines {
            Self::remove_timestamp(&mut line);
            if line.len() > 0 {
                let tokens: Vec<&str> = line.split("#: ").collect();
                if tokens.len() == 2 {
                    return tokens[1].to_string();
                }
            }
        }
        panic!("File with log parsing start token");
    }

    fn parse_file(mut self) -> Model {
        let input_file_buffer = self.input_file_buffer.take().unwrap();
        let mut lines_it = input_file_buffer.lines();

        self.sequence_name = Self::move_to_begin_token(&mut lines_it);
        
        for mut line in lines_it {
            Self::remove_timestamp(&mut line);
            if Self::is_end_line(&line, &self.sequence_name) {
                self.model.set_sequence_name(self.sequence_name);
                return self.model;
            }

            if line.len() > 0 {
                match self.capturing_state {
                    RecordCapturingState::OneShot => self.one_shot_state(line),
                    RecordCapturingState::Multiline => self.multiline_capture_state(line),
                    RecordCapturingState::Ranged => self.ranged_capture_state(line),
                }
            }
        }
        panic!("End of file before end token");
    }

}

fn main() {
    let mut args = env::args();
    let _path = args.next();
    let input_file_path = args.next().unwrap();
    let input_file = File::open(input_file_path).unwrap();

    let output_file_path = args.next().unwrap();
    let output_file = File::create(output_file_path).unwrap();

    let mut converter = Converter::new(input_file);
    let model = converter.parse_file();
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
[   62.996339] #= Cmd52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
[   64.049750] #= Cmd53 write: bool(true) fn: u8(1) add: x32(0x01043) inc: bool(true)
[   64.054214] data: x8([0x80,0x05]) =#
[   62.996339] #= Cmd52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
Just trash
[   64.049750] #= Cmd53 write: bool(false) fn: u8(0) add: x32(0x01043) inc: bool(true)
[   64.054214] data: x8([0x80,0x05,
[   64.054214] 0x20,0xfe,0xc4,
[   64.054214] 0x31,0x4,0x60,0xce]) =#
[   58.432667] #[ Read_Efuse
[   58.437030] #- offset: x16(0x5ea) efuse_start: x8(0x0) size: u8(1) read_efuse_cnt: u32(10000) efuse_ctrl: x8(0x30) dv_sel: id(EfuseAccess::DDV)\n
[   62.996339] #= Cmd52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
[   58.439921] #- map_ptr: x64(00000000da5708c1)
[   64.592339] I am not a record
Me neither
[   64.921023] Read_Efuse #]
[   62.996339] #= Another_Cmd num: f32(3.565) adv: id(EfuseAccess::DAV) top: i32(-2500) adu: id(EfuseAccess::DXV) dot: i8([-25,-69,2]) =#
[   64.921023] test_sequence :#
[   62.996339] #= Unexisting_Cmd write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#\n";

        let test_files_dir = "/tmp".to_string();
        let test_input_file_path = test_files_dir.clone() + "/test_input_file";

        let mut test_input_file = File::create(test_input_file_path.clone()).unwrap();
        test_input_file
            .write_all(test_file_content.as_bytes())
            .unwrap();
        drop(test_input_file);

        let test_input_file = File::open(test_input_file_path.clone()).unwrap();

        let test_output_source_file_path = test_files_dir.clone() + "/test_output_file.c";
        let mut test_output_source_file = File::create(test_output_source_file_path).unwrap();

        let test_output_header_file_path = test_files_dir.clone() + "/test_output_file.h";
        let mut test_output_header_file = File::create(test_output_header_file_path).unwrap();

        let converter = Converter::new(test_input_file);
        let mut model = converter.parse_file();
         model.compute_to_c(&mut test_output_source_file, &mut test_output_header_file);

    }
}
