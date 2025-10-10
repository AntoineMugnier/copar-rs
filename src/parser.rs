use crate::model;
use crate::{model::Model, unirecord::RecordParsingError};
use std::io::Read;
use std::str::Lines;
use std::{char, fs::File};

use crate::unirecord::{UniRecord, UniRecordArgVariant};
enum RecordCapturingState {
    OneShot,
    Multiline,
    Ranged,
}

pub type ParserResult<T> = Result<T, LineParsingError>;

#[derive(Debug)]
pub enum LineParsingError {
    MultipleRecordDelimiters(String),
    UnmatchedNameSequenceStop(Box<(String, String)>),
    BadRecordArg(RecordParsingError),
    UnmatchedRangedRecordName(Box<(String, String)>),
    UncompleteRecordArg,
    UnmatchingRecordDelimiters(char, char),
    MissingEndDelimiter(char),
    MissingBeginDelimiter(char),
    UnparsableLine,
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
    NoSequenceEnd {
        last_record_start_line: usize,
    },
}

// Copar command parser
pub struct Parser {
    sequence_name: String,
    input_file_buffer: Option<String>,
    model: Model,
    capturing_state: RecordCapturingState,
    last_record_start_line: usize,
    current_line: usize,
    line_buffer: String,
}

impl Parser {
    // Instanciate this structure with the log file containing the copar records
    pub fn new(mut input_file: File) -> Parser {
        let mut input_file_buffer = String::new();
        input_file.read_to_string(&mut input_file_buffer).unwrap();

        Parser {
            sequence_name: String::new(),
            input_file_buffer: Some(input_file_buffer),
            model: model::Model::default(),
            capturing_state: RecordCapturingState::OneShot,
            last_record_start_line: 0,
            current_line: 0,
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
                .map_err(LineParsingError::BadRecordArg)?;
            uni_record_args.push(uni_record_arg);
        }
        Ok(uni_record_args)
    }

    fn parse_line_buffer(&mut self) -> ParserResult<()> {
        let record_str_vec: Vec<&str> = self.line_buffer.splitn(2, ' ').collect();
        let name = String::from(record_str_vec[0]);
        let mut args = Vec::new();
        if record_str_vec.len() >= 2 {
            args = Self::parse_record_args(record_str_vec[1])?;
        }

        let uni_record = UniRecord::new(name, args);
        self.model.add_record(uni_record);
        Ok(())
    }

    fn one_shot_state(&mut self, mut line: &str) -> ParserResult<bool> {
        let res = Self::get_delimited_content(&mut line);
        let mut is_end_of_sequence = false;
        match res {
            Ok(delimiter_token_char) => match delimiter_token_char {
                '=' => {
                    self.last_record_start_line = self.current_line;
                    self.line_buffer = line.to_string();
                    self.parse_line_buffer()?;
                }
                '[' => {
                    self.last_record_start_line = self.current_line;
                    self.line_buffer = line.to_string();
                    self.capturing_state = RecordCapturingState::Ranged;
                }
                '>' => {
                    let read_sequence_name = line;
                    if read_sequence_name != self.sequence_name {
                        return Err(LineParsingError::UnmatchedNameSequenceStop(Box::new((
                            String::from(read_sequence_name),
                            self.sequence_name.clone(),
                        ))));
                    }
                    is_end_of_sequence = true;
                }
                _ => (),
            },
            Err(LineParsingError::MissingEndDelimiter(delimiter_token_char)) => {
                match delimiter_token_char {
                    '=' => {
                        self.line_buffer = line.to_string();
                        self.capturing_state = RecordCapturingState::Multiline;
                    }
                    '[' => return Err(LineParsingError::MissingEndDelimiter(delimiter_token_char)),
                    _ => (),
                }
            }
            Err(LineParsingError::UnparsableLine) => (),
            Err(e) => return Err(e),
        }

        Ok(is_end_of_sequence)
    }

    fn multiline_capture_state(&mut self, mut line: &str) -> ParserResult<()> {
        let res = Self::get_delimited_content(&mut line);
        match res {
            Ok(delimiter_token_char) => {
                if let '=' = delimiter_token_char {
                    // Add space if last line was uncut
                    if self.line_buffer.ends_with(')') {
                        self.line_buffer += " ";
                    }
                    self.line_buffer += line;
                    self.parse_line_buffer()?;
                    self.capturing_state = RecordCapturingState::OneShot;
                }
            }
            Err(LineParsingError::MissingBeginDelimiter(delimiter_token_char)) => {
                if let '=' = delimiter_token_char {
                    // Add space if last line was uncut
                    if self.line_buffer.ends_with(')') {
                        self.line_buffer += " ";
                    }
                    self.line_buffer += line;
                    self.parse_line_buffer()?;
                    self.capturing_state = RecordCapturingState::OneShot;
                }
            }
            Err(LineParsingError::UnparsableLine) => {
                // Add space if last line was uncut
                if self.line_buffer.ends_with(')') {
                    self.line_buffer += " ";
                }
                self.line_buffer += line;
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn compare_record_name_to_ref(&mut self, line: &mut &str) -> ParserResult<()> {
        let tokens: Vec<&str> = line.splitn(2, " ").collect();
        let record_name = tokens[0];
        let previous_record_name = self.line_buffer.split(" ").next().unwrap();
        if record_name != previous_record_name {
            return Err(LineParsingError::UnmatchedRangedRecordName(Box::new((
                previous_record_name.to_string(),
                record_name.to_string(),
            ))));
        }
        if tokens.len() == 2 {
            *line = tokens[1];
        }
        Ok(())
    }

    fn ranged_capture_state(&mut self, mut line: &str) -> ParserResult<()> {
        let res = Self::get_delimited_content(&mut line);

        if let Ok(delimiter_token_char) = res {
            match delimiter_token_char {
                '-' => {
                    if let Ok(()) = self.compare_record_name_to_ref(&mut line) {
                        self.line_buffer += " ";
                        self.line_buffer += line;
                    }
                }
                ']' => {
                    if let Ok(()) = self.compare_record_name_to_ref(&mut line) {
                        self.capturing_state = RecordCapturingState::OneShot;
                        self.parse_line_buffer()?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn get_delimited_content(line_content: &mut &str) -> ParserResult<char> {
        let mut has_begin_token = false;
        let mut has_end_token = false;
        let mut begin_delimiter_token_char = ' ';
        let mut end_delimiter_token_char = ' ';
        let mut range_index_start = 0;
        let mut range_index_stop = line_content.len();

        let tokens: Vec<&str> = line_content.split(" ").collect();
        let begin_delimiter = tokens[0];
        let begin_delimiter_tokens: Vec<&str> = begin_delimiter.split('#').collect();
        if begin_delimiter_tokens.len() == 2 {
            let delimiter_token = begin_delimiter_tokens[1];
            if delimiter_token.chars().count() == 1 {
                range_index_start += begin_delimiter.len() + 1;
                begin_delimiter_token_char = delimiter_token.chars().next().unwrap();
                has_begin_token = true;
            }
        }

        let end_delimiter = tokens.last().unwrap();
        let end_delimiter_tokens: Vec<&str> = end_delimiter.split('#').collect();
        if end_delimiter_tokens.len() == 2 {
            let delimiter_token = end_delimiter_tokens[0];
            if delimiter_token.chars().count() == 1 {
                end_delimiter_token_char = delimiter_token.chars().next().unwrap();
                range_index_stop -= end_delimiter.len() + 1;
                has_end_token = true;
            }
        }

        let result_line = &line_content[range_index_start..range_index_stop];
        match (has_begin_token, has_end_token) {
            (false, false) => Err(LineParsingError::UnparsableLine),
            (false, true) => {
                *line_content = result_line;
                Err(LineParsingError::MissingBeginDelimiter(
                    end_delimiter_token_char,
                ))
            }
            (true, false) => {
                *line_content = result_line;
                Err(LineParsingError::MissingEndDelimiter(
                    begin_delimiter_token_char,
                ))
            }
            (true, true) => {
                *line_content = result_line;
                if begin_delimiter_token_char != end_delimiter_token_char {
                    return Err(LineParsingError::UnmatchingRecordDelimiters(
                        begin_delimiter_token_char,
                        end_delimiter_token_char,
                    ));
                }
                Ok(begin_delimiter_token_char)
            }
        }
    }

    fn move_to_begin_token(
        lines: &mut Lines,
        line_index_ref: &mut usize,
    ) -> FileParsingResult<String> {
        for (line_index, mut line_content) in lines.enumerate() {
            Self::remove_timestamp(&mut line_content);
            if !line_content.is_empty() {
                let res = Self::get_delimited_content(&mut line_content);
                match res {
                    Ok(delimiter_char) => {
                        if delimiter_char == '<' {
                            *line_index_ref += line_index;
                            return FileParsingResult::Ok(line_content.to_string());
                        }
                    }
                    Err(LineParsingError::UnparsableLine) => (),
                    Err(e) => {
                        return Err(FileParsingError::LineError {
                            line_nb: line_index + 1,
                            line_error: e,
                        });
                    }
                }
            }
        }

        Err(FileParsingError::NoSequenceStart)
    }

    /// Parse the file, returning a valid copar model
    pub fn parse_file(mut self) -> Result<Model, FileParsingError> {
        let input_file_buffer = self.input_file_buffer.take().unwrap();
        let mut lines_it = input_file_buffer.lines();
        let mut line_index_offset = 1;
        let mut end_of_parsing = false;
        self.sequence_name = Self::move_to_begin_token(&mut lines_it, &mut line_index_offset)?;

        for (line_index, mut line_content) in lines_it.enumerate() {
            self.current_line = line_index + line_index_offset + 1;
            Self::remove_timestamp(&mut line_content);

            if !line_content.is_empty() {
                let res = match self.capturing_state {
                    RecordCapturingState::OneShot => {
                        let line_parsing_err = self.one_shot_state(line_content);
                        line_parsing_err.map(|end_of_parsing_| end_of_parsing = end_of_parsing_)
                    }
                    RecordCapturingState::Multiline => self.multiline_capture_state(line_content),

                    RecordCapturingState::Ranged => self.ranged_capture_state(line_content),
                };

                res.map_err(|e| FileParsingError::LineError {
                    line_nb: self.current_line,
                    line_error: e,
                })?;

                if end_of_parsing {
                    self.model.set_sequence_name(self.sequence_name);
                    return Ok(self.model);
                }
            }
        }
        Err(FileParsingError::NoSequenceEnd {
            last_record_start_line: self.last_record_start_line,
        })
    }
}
