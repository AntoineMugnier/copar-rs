pub mod c_generation;
pub mod c_sharp_generation;
pub mod rust_generation;

pub(crate) fn generate_blank_line(output_file: &mut impl std::io::Write) {
    writeln!(output_file).unwrap();
}
