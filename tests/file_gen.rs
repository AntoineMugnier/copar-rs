mod test_files;
use copar::CGeneration;
use copar::Parser;
use std::{
    fs::File,
    io::{Read, Write},
};

use test_files::{TEST_FILE_C_CONTENT, TEST_FILE_H_CONTENT, TEST_FILE_LOG};
#[test]
fn test_gen() {
    let test_files_dir = "/tmp".to_string();
    let test_input_file_path = test_files_dir.clone() + "/test_input_file";

    let mut test_input_file = File::create(test_input_file_path.clone()).unwrap();
    test_input_file.write_all(TEST_FILE_LOG.as_bytes()).unwrap();
    drop(test_input_file);

    let test_input_file = File::open(test_input_file_path.clone()).unwrap();

    let test_output_source_file_path = test_files_dir.clone() + "/test_output_file.c";
    let mut test_output_source_file = File::create(test_output_source_file_path.as_str()).unwrap();

    let test_output_header_file_path = test_files_dir.clone() + "/test_output_file.h";
    let mut test_output_header_file = File::create(test_output_header_file_path.as_str()).unwrap();

    let converter = Parser::new(test_input_file);
    let mut model = converter.parse_file().unwrap();
    model.compute_to_c(&mut test_output_source_file, &mut test_output_header_file);

    let open_file = |file_path: &str| -> String {
        let mut c_file_obtained = File::open(file_path).unwrap();
        let mut c_file_content_obtained = String::new();
        c_file_obtained
            .read_to_string(&mut c_file_content_obtained)
            .unwrap();
        c_file_content_obtained
    };

    let c_file_buffer_obt = open_file(&test_output_source_file_path);
    let h_file_buffer_obt = open_file(&test_output_header_file_path);

    let c_file_buffer_exp = TEST_FILE_C_CONTENT;
    let h_file_buffer_exp = TEST_FILE_H_CONTENT;

    assert_eq!(c_file_buffer_exp, c_file_buffer_obt);
    assert_eq!(h_file_buffer_exp, h_file_buffer_obt);
}
