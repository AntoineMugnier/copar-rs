use crate::model::Model;
use std::fs::File;
use std::io::Read;
use std::str::Lines;

use crate::unirecord::{UniRecord, UniRecordArgVariant};
enum RecordCapturingState {
    OneShot,
    Multiline,
    Ranged,
}

pub struct Parser {
    sequence_name: String,
    input_file_buffer: Option<String>,
    model: Model,
    capturing_state: RecordCapturingState,
    line_buffer: String,
}

impl Parser {
    pub fn new(mut input_file: File) -> Parser {
        let mut input_file_buffer = String::new();
        input_file.read_to_string(&mut input_file_buffer).unwrap();

        Parser {
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

    fn parse_record_args(line: &str) -> Vec<UniRecordArgVariant> {
        let mut uni_record_args = Vec::new();
        for args in line.split(' ').collect::<Vec<&str>>().windows(2).step_by(2) {
            let arg_name = args[0];
            let arg_val = args[1];
            let uni_record_arg = UniRecordArgVariant::from(arg_name, arg_val).unwrap();
            uni_record_args.push(uni_record_arg);
        }
        uni_record_args
    }

    fn parse_line_buffer(&mut self) {
        let record_str_vec: Vec<&str> = self.line_buffer.splitn(2, ' ').collect();

        let name = String::from(record_str_vec[0]);
        let args = Self::parse_record_args(record_str_vec[1]);
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
                    self.parse_line_buffer();
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
        if self.line_buffer.chars().last().unwrap() == ')' {
            self.line_buffer += " ";
        }
        self.line_buffer += tokens[0];

        if tokens.len() == 2 {
            self.parse_line_buffer();
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
            self.parse_line_buffer();
            self.capturing_state = RecordCapturingState::OneShot;
        }
    }
    fn is_end_line(line: &str, sequence_name: &String) -> bool {
        let tokens: Vec<&str> = line.split(" :#").collect();

        assert!(tokens.len() <= 2);

        if tokens.len() == 2 {
            let read_sequence_name = tokens[0];
            assert_eq!(read_sequence_name, sequence_name);
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

    pub fn parse_file(mut self) -> Model {
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
