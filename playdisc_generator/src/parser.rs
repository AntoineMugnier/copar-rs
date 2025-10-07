use crate::{model::Model, unirecord::RecordParsingError};
use std::fs::File;
use std::io::Read;
use std::str::Lines;

use crate::unirecord::{UniRecord, UniRecordArgVariant};
enum RecordCapturingState {
    OneShot,
    Multiline,
    Ranged,
}

type ParserResult<T> = Result<T, LineParsingError>;

#[derive(Debug)]
pub enum LineParsingError {
    MultipleRecordDelimiters(String),
    UnmatchedNameSequenceStop(Box<(String, String)>),
    BadRecordArg(RecordParsingError),
    UnmatchedRangedRecordName(Box<(String, String)>),
    UncompleteRecordArg,
    MissingEndDelimiter(String),
    MissingRecordArgs,
}

type FileParsingResult<T> = Result<T, FileParsingError>;
#[derive(Debug)]
pub enum FileParsingError {
    LineError {
        line_nb: usize,
        line_error: LineParsingError,
    },
    NoSequenceStart,
    NoSequenceEnd,
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

    fn parse_record_args(line: &str) -> ParserResult<Vec<UniRecordArgVariant>> {
        let mut uni_record_args = Vec::new();
        let args: Vec<&str> = line.split(' ').collect();
        if args.len() % 2 != 0 {
            return Err(LineParsingError::UncompleteRecordArg);
        }
        for arg in args.windows(2).step_by(2) {
            let arg_name = arg[0];
            let arg_val = arg[1];
            let uni_record_arg = UniRecordArgVariant::from(arg_name, arg_val)
                .map_err(|e| LineParsingError::BadRecordArg(e))?;
            uni_record_args.push(uni_record_arg);
        }
        Ok(uni_record_args)
    }

    fn parse_line_buffer(&mut self) -> ParserResult<()> {
        let record_str_vec: Vec<&str> = self.line_buffer.splitn(2, ' ').collect();

        let name = String::from(record_str_vec[0]);
        let args = Self::parse_record_args(record_str_vec[1])?;
        let uni_record = UniRecord::new(name, args);

        self.model.add_record(uni_record);
        Ok(())
    }

    fn one_shot_state(&mut self, line: &str) -> ParserResult<()> {
        let tokens: Vec<&str> = line.splitn(2, [' ']).collect();
        match tokens[0] {
            "#=" => {
                if tokens.len() <= 1 {
                    return Err(LineParsingError::MissingRecordArgs);
                }
                if tokens[1].split("#=").count() > 1 {
                    return Err(LineParsingError::MultipleRecordDelimiters(String::from(
                        "#= ",
                    )));
                }

                let sub_tokens: Vec<&str> = tokens[1].split(" =#").collect();
                self.line_buffer = sub_tokens[0].to_string();

                if tokens.len() > 2 {
                    return Err(LineParsingError::MultipleRecordDelimiters(String::from(
                        " =#",
                    )));
                }
                if sub_tokens.len() == 2 {
                    self.parse_line_buffer()?;
                } else {
                    self.capturing_state = RecordCapturingState::Multiline;
                }
            }
            "#[" => {
                if tokens.len() <= 1 {
                    return Err(LineParsingError::MissingRecordArgs);
                }
                if tokens[1].split("#[").count() > 1 {
                    return Err(LineParsingError::MultipleRecordDelimiters(String::from(
                        "#[ ",
                    )));
                }
                self.line_buffer = String::new();
                self.line_buffer = tokens[1].to_string();
                self.capturing_state = RecordCapturingState::Ranged;
            }
            _ => {
                // Unparsable line, skip
            }
        }
        Ok(())
    }

    fn multiline_capture_state(&mut self, line: &str) -> ParserResult<()> {
        // Add space if last line was uncut
        if self.line_buffer.chars().last().unwrap() == ')' {
            self.line_buffer += " ";
        }

        let tokens: Vec<&str> = line.split(" =#").collect();
        self.line_buffer += tokens[0];

        if tokens.len() == 2 {
            self.parse_line_buffer()?;
            self.capturing_state = RecordCapturingState::OneShot;
        } else if tokens.len() > 2 {
            return Err(LineParsingError::MultipleRecordDelimiters(String::from(
                " =#",
            )));
        }

        return Ok(());
    }

    fn ranged_capture_state(&mut self, line: &str) -> ParserResult<()> {
        let tokens: Vec<&str> = line.split("#- ").collect();
        if tokens.len() == 2 {
            let sub_tokens: Vec<&str> = tokens[1].split(" -#").collect();
            let nb_sub_tokens = sub_tokens.len();
            if nb_sub_tokens == 2 {
                self.line_buffer += " ";
                self.line_buffer += sub_tokens[0];
                return Ok(());
            }
            else if nb_sub_tokens == 1 {
                return Err(LineParsingError::MissingEndDelimiter(String::from(" -#")));
            }
            else{
            return Err(LineParsingError::MultipleRecordDelimiters(String::from(
                " -#",
            )));
            }

        } else if tokens.len() > 2 {
            return Err(LineParsingError::MultipleRecordDelimiters(String::from(
                "#- ",
            )));
        }

        let tokens: Vec<&str> = line.split(" ]#").collect();
        if tokens.len() == 2 {
            let record_name = tokens[0];
            let previous_record_name = self.line_buffer.split(" ").next().unwrap();
            if record_name != previous_record_name {
                return Err(LineParsingError::UnmatchedRangedRecordName(Box::new((
                    previous_record_name.to_string(),
                    record_name.to_string(),
                ))));
            }
            self.parse_line_buffer()?;
            self.capturing_state = RecordCapturingState::OneShot;
        }
        Ok(())
    }

    fn is_end_line(line: &str, sequence_name: &String) -> ParserResult<bool> {
        let tokens: Vec<&str> = line.split(" :#").collect();

        if tokens.len() > 2 {
            return Err(LineParsingError::MultipleRecordDelimiters(String::from(
                " :#",
            )));
        };

        if tokens.len() == 2 {
            let read_sequence_name = tokens[0];
            if read_sequence_name != sequence_name {
                return Err(LineParsingError::UnmatchedNameSequenceStop(Box::new((
                    String::from(read_sequence_name),
                    sequence_name.clone(),
                ))));
            }
            return Ok(true);
        }

        return Ok(false);
    }

    fn move_to_begin_token(lines: &mut Lines, line_index_ref: &mut usize) -> FileParsingResult<String> {
        for (line_index, mut line_content) in lines.enumerate() {
            Self::remove_timestamp(&mut line_content);
            if line_content.len() > 0 {
                let tokens: Vec<&str> = line_content.split("#: ").collect();
                if tokens.len() == 2 {
                    *line_index_ref = line_index;
                    return Ok(tokens[1].to_string());
                } else if tokens.len() > 2 {
                    let line_error = LineParsingError::MultipleRecordDelimiters(String::from("#: "));
                    return Err(FileParsingError::LineError {
                        line_nb: line_index + 1,
                        line_error,
                    });
                }
            }
        }
        Err(FileParsingError::NoSequenceStart)
    }

    pub fn parse_file(mut self) -> Result<Model, FileParsingError> {
        let input_file_buffer = self.input_file_buffer.take().unwrap();
        let mut lines_it = input_file_buffer.lines();
        let mut line_index_offset = 0;
        self.sequence_name = Self::move_to_begin_token(&mut lines_it, &mut line_index_offset)?;

        for (line_index, mut line_content) in lines_it.enumerate() {
            Self::remove_timestamp(&mut line_content);
            let err = Self::is_end_line(&line_content, &self.sequence_name);

            if err.map_err(|e| FileParsingError::LineError {
                line_nb : line_index + line_index_offset + 1,
                line_error: e,
            })?{
                self.model.set_sequence_name(self.sequence_name);
                return Ok(self.model);
            }

            if line_content.len() > 0 {
                let err;

                match self.capturing_state {
                    RecordCapturingState::OneShot => err = self.one_shot_state(line_content),
                    RecordCapturingState::Multiline => {
                        err = self.multiline_capture_state(line_content)
                    }

                    RecordCapturingState::Ranged => err = self.ranged_capture_state(line_content),
                }

                err.map_err(|e| FileParsingError::LineError {
                    line_nb : line_index + line_index_offset + 1,
                    line_error: e,
                })?;
            }
        }
        return Err(FileParsingError::NoSequenceEnd);
    }
}
