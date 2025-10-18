mod test_files;
use copar::{CGeneration, Model, RustGeneration};

use test_files::{TEST_FILE_C_CONTENT, TEST_FILE_H_CONTENT, TEST_FILE_LOG};

use crate::test_files::TEST_FILE_RUST_CONTENT;

#[test]
fn test_c_gen() {
    let model = Model::parse(TEST_FILE_LOG.as_bytes()).unwrap();
    let mut test_output_file_c = Vec::new();
    let mut test_output_file_h = Vec::new();
    model.compute_to_c(&mut test_output_file_c, &mut test_output_file_h);

    assert_eq!(
        String::from_utf8(test_output_file_c).unwrap(),
        TEST_FILE_C_CONTENT
    );

    assert_eq!(
        String::from_utf8(test_output_file_h).unwrap(),
        TEST_FILE_H_CONTENT
    );
}

#[test]
fn test_rust_gen() {
    let model = Model::parse(TEST_FILE_LOG.as_bytes()).unwrap();
    let mut test_output_file_rs = Vec::new();
    model.compute_to_rust(&mut test_output_file_rs);
    println!(
        "{}",
        String::from_utf8(test_output_file_rs.clone()).unwrap()
    );

    assert_eq!(
        String::from_utf8(test_output_file_rs).unwrap(),
        TEST_FILE_RUST_CONTENT
    );
}
