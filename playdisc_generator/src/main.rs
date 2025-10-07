use std::{env, fs::File};

use playdisc_generator::Parser;

fn main() {
    let mut args = env::args();
    let _path = args.next();
    let input_file_path = args.next().unwrap();
    let input_file = File::open(input_file_path).unwrap();

    let output_c_file_path = args.next().unwrap();
    let mut output_c_file = File::create(output_c_file_path).unwrap();

    let output_h_file_path = args.next().unwrap();
    let mut output_h_file = File::create(output_h_file_path).unwrap();

    let converter = Parser::new(input_file);
    let mut model = converter.parse_file();
    model.compute_to_c(&mut output_c_file, &mut output_h_file);
}
