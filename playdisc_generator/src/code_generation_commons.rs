use std::{fs::File, io::Write};

pub(crate) fn generate_blank_line(output_file: &mut File) {
    write!(output_file, "\n").unwrap();
}

pub(crate) fn pascal_to_snake_case(input: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}
pub(crate) fn pascal_to_macro_case(input: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c.to_ascii_uppercase());
        }
    }

    result
}